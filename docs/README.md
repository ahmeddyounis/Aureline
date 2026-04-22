# Docs index

Aureline is an open-source next-generation IDE (working name). The
repository is in its pre-implementation stage; these documents describe
the governance, ownership, and build discipline that precede source
code.

## Governance

- [`governance/dri_map.md`](./governance/dri_map.md) — DRI, backup
  owners, blocker aging, and narrowing authority.
- [`governance/maintainer_coverage_policy.md`](./governance/maintainer_coverage_policy.md)
  — reviewer-depth, backup-owner, signing-quorum, and critical-upstream
  linkage rules for protected paths.
- [`governance/control_artifact_index.md`](./governance/control_artifact_index.md)
  — overview of the control-artifact index: one home, one owner,
  and one review path for every control asset.
- [`governance/requirement_alias_crosswalk.md`](./governance/requirement_alias_crosswalk.md)
  — canonical requirement-id crosswalk for milestone packets, CI,
  waivers, scorecards, docs, and release evidence.
- [`governance/interface_inventory.md`](./governance/interface_inventory.md)
  — outline of interface-inventory categories and owning lanes.
- [`governance/interface_lifecycle_policy.md`](./governance/interface_lifecycle_policy.md)
  — shared lifecycle and deprecation metadata policy for stable ids,
  aliases, schema families, replacement chains, support windows, and
  notice surfaces. Boundary schema in
  [`/schemas/governance/deprecation_metadata.schema.json`](../schemas/governance/deprecation_metadata.schema.json);
  worked rows in
  [`/fixtures/governance/deprecation_examples/`](../fixtures/governance/deprecation_examples/).
- [`governance/interface_freeze_matrix.md`](./governance/interface_freeze_matrix.md)
  — implementation-broadening freeze matrix covering which contracts
  are frozen, provisional, or blocked before scope widens.
- [`governance/interface_freeze_guide.md`](./governance/interface_freeze_guide.md)
  — short citation guide for downstream tasks that should reference
  frozen rows instead of repeating contract prose.
- [`governance/benchmark_council_charter.md`](./governance/benchmark_council_charter.md)
  — seed charter for the benchmark council (roles, scope, cadence,
  quorum placeholder, escalation).
- [`governance/feature_flag_policy.md`](./governance/feature_flag_policy.md)
  — normative policy for experiments, feature flags, Labs inventory,
  rollout rows, policy disables, and kill switches before a runtime
  control plane exists.
- [`governance/decision_backlog.md`](./governance/decision_backlog.md)
  — seeded architecture decisions with freeze dates and default
  narrowing postures.
- [`governance/decision_workflow.md`](./governance/decision_workflow.md)
  — how decisions open, close, supersede, and narrow.
- [`governance/commitment_and_rebaseline_policy.md`](./governance/commitment_and_rebaseline_policy.md)
  — commitment classes, assumption-invalidating events, phase-level
  change budgets, and exception-packet rules.
- [`../artifacts/governance/dependency_ledger.yaml`](../artifacts/governance/dependency_ledger.yaml)
  — canonical blocker ids for protected M0/M1 lanes and backlog rows,
  with latest safe decision points and fallback posture.
- [`governance/blocker_aging_slas.md`](./governance/blocker_aging_slas.md)
  — blocker-aging and escalation table for architecture-freeze
  blockers, stale evidence, owner gaps, and unresolved waivers.
- [`../artifacts/governance/correction_trigger_table.yaml`](../artifacts/governance/correction_trigger_table.yaml)
  — scorecard and risk-linked correction triggers showing when a slip
  forces descoping, rebaseline, or an exception packet.
- [`governance/descoping_policy.md`](./governance/descoping_policy.md)
  — canonical descoping ladder, never-cut bars, milestone-at-risk
  defaults, and repeated-miss routing.
- [`../artifacts/milestones/cut_classes.yaml`](../artifacts/milestones/cut_classes.yaml)
  and [`../artifacts/milestones/kill_criteria.yaml`](../artifacts/milestones/kill_criteria.yaml)
  — machine-readable backlog/requirement cut classes and protected
  quality kill rows.
- [`governance/change_budget_workflow.md`](./governance/change_budget_workflow.md)
  — protected-path change-budget matrix, exception-packet workflow,
  repeated-exception thresholds, and dashboard-feed fields for
  freeze-era decisions.
- [`governance/dogfood_issue_taxonomy.md`](./governance/dogfood_issue_taxonomy.md)
  — dogfood intake taxonomy covering category, severity, evidence-link,
  exact-build, route-truth, and hidden-dependency fields for issue
  templates and intake automation.
- [`governance/verification_packet_template.md`](./governance/verification_packet_template.md)
  — canonical verification-packet template with shared claim-row,
  evidence-id, freshness, and signoff structure.
- [`../artifacts/governance/evidence_id_conventions.md`](../artifacts/governance/evidence_id_conventions.md)
  — stable evidence-id grammar and artifact-linking rules across
  design, benchmark, verification, support, and signoff packets.
- [`governance/templates/`](./governance/templates/) — waiver and
  exception-packet templates plus legacy freeze-exception compatibility
  aliases.
- [`governance/provenance_and_compliance_baseline.md`](./governance/provenance_and_compliance_baseline.md)
  — IP, provenance, and supply-chain baseline that pairs with
  [`/CONTRIBUTING.md`](../CONTRIBUTING.md).
- [`architecture/build_vs_buy_register.md`](./architecture/build_vs_buy_register.md)
  — canonical launch-critical build-vs-buy register, scoring rubric,
  and dependency traceability for protected-path selections.
- [`governance/dependency_review_policy.md`](./governance/dependency_review_policy.md)
  — dependency/import admission policy, build-vs-buy linkage rules,
  stale-row thresholds, and notice/SBOM/provenance flow.
- [`governance/fork_review_policy.md`](./governance/fork_review_policy.md)
  — review bar for protected-path forks, long-lived patch stacks, exit
  strategy, and re-upstream planning.
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
- [`governance/record_class_governance.md`](./governance/record_class_governance.md)
  — record-class registry seed for telemetry schemas, crash and support
  evidence, collaboration evidence, AI retained evidence, entitlement
  and usage export packets, exit packets, and destruction receipts.
  Machine-readable registry in
  [`/artifacts/governance/record_class_registry.yaml`](../artifacts/governance/record_class_registry.yaml);
  boundary schema in
  [`/schemas/governance/record_class.schema.json`](../schemas/governance/record_class.schema.json).

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
- [`ci/control_artifact_validation.md`](./ci/control_artifact_validation.md)
  — local and CI contract-artifact validation lane for package,
  boundary, claim, control-index, and decision/source-anchor drift.
- [`build/exact_build_identity_model.md`](./build/exact_build_identity_model.md)
  — exact-build identity model and per-surface propagation rules for
  release, docs/help, benchmark, and support/export truth.
- [`build/reproducible_build_baseline.md`](./build/reproducible_build_baseline.md)
  — pinned toolchain, bootstrap command, and build-identity record.
- [`build/cleanroom_rebuild_lane.md`](./build/cleanroom_rebuild_lane.md)
  — first clean-room rebuild lane, emitted input-manifest shape,
  artifact-digest comparison rules, and named reproducibility gaps.

## Release and benchmark publication

- [`adr/0017-release-posture-artifact-families-and-promotion-gates.md`](./adr/0017-release-posture-artifact-families-and-promotion-gates.md)
  — release-governance ADR covering channel posture, RC-as-stage,
  rollback atom, same-change-set release bundles, waiver and
  late-proof policy, emergency mirror/manual-import transport, and the
  promotion vetoes that still block RC/stable, LTS, and hotfix
  widening after a binary build succeeds. Machine-readable companions
  in
  [`/artifacts/release/artifact_family_map.yaml`](../artifacts/release/artifact_family_map.yaml)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../artifacts/release/promotion_gate_map.yaml).
- [`release/release_artifact_graph.md`](./release/release_artifact_graph.md)
  — publishable release-artifact graph, bundle-completeness rules, and
  contract-surface index.
- [`release/release_evidence_packet_template.md`](./release/release_evidence_packet_template.md)
  — release-truth packet template and waiver-aware shiproom structure.
- [`release/shiproom_runbook.md`](./release/shiproom_runbook.md)
  — canonical shiproom review order, go/no-go vocabulary, exception
  logging, and release/support/docs/security/architecture handoffs.
- [`benchmarks/benchmark_publication_pack_template.md`](./benchmarks/benchmark_publication_pack_template.md)
  — public benchmark/public-proof packet template with exact command
  line, comparability, protected-metrics revision, docs applicability,
  exclusion, and competitor disclosure fields.
- [`benchmarks/public_comparison_rules.md`](./benchmarks/public_comparison_rules.md)
  — methodology-only versus claim-bearing publication rules and
  head-to-head comparison disclosure requirements.

## Platform conformance

- [`adr/0016-shell-windowing-input-accessibility-boundary.md`](./adr/0016-shell-windowing-input-accessibility-boundary.md)
  — desktop shell boundary ADR covering canonical shell zones,
  adaptive classes, command-entry routes, text-input normalization,
  accessibility-tree ownership, and restore-vs-rebind rules.
  Machine-readable companion in
  [`/artifacts/ux/desktop_shell_boundary_matrix.yaml`](../artifacts/ux/desktop_shell_boundary_matrix.yaml);
  degraded-state companion in
  [`/docs/architecture/input_adapter_failure_modes.md`](./architecture/input_adapter_failure_modes.md).
- [`platform/desktop_platform_conformance_matrix.md`](./platform/desktop_platform_conformance_matrix.md)
  — claimed macOS, Windows, and Linux desktop profile roster plus the
  per-surface owner, validation method, release bar, deployment-path
  narrowing, and explicitly unclaimed lane matrix that release, support,
  accessibility, and compatibility packets cite.
  Machine-readable companion in
  [`/artifacts/platform/claimed_desktop_profiles.yaml`](../artifacts/platform/claimed_desktop_profiles.yaml).
- [`qa/multi_window_verification.md`](./qa/multi_window_verification.md)
  — seeded verification matrix for multi-window, monitor-topology,
  mixed-DPI, suspend/resume, off-screen recovery, and restart/reopen
  continuity on named desktop profiles. Machine-readable seed in
  [`/artifacts/qa/window_display_matrix.yaml`](../artifacts/qa/window_display_matrix.yaml);
  concrete cases in
  [`/fixtures/platform/window_display_cases/`](../fixtures/platform/window_display_cases/).
- [`accessibility/a11y_ime_packet_template.md`](./accessibility/a11y_ime_packet_template.md)
  — reviewer-facing packet template for launch-critical accessibility
  and platform-input evidence. Machine-readable companions in
  [`/artifacts/accessibility/`](../artifacts/accessibility/); concrete
  IME and text cases in
  [`/fixtures/accessibility/ime_and_text_cases/`](../fixtures/accessibility/ime_and_text_cases/).

## Command contracts

- [`commands/command_descriptor_contract.md`](./commands/command_descriptor_contract.md)
  — canonical command object and invocation-session packet contract for
  palette, menu, CLI/help, AI-tool, automation, and replay or audit
  surfaces.
- [`commands/command_graph_and_ui_slots_seed.md`](./commands/command_graph_and_ui_slots_seed.md)
  — slot-taxonomy and projection rules that translate descriptor
  discoverability into concrete shell slots and help surfaces.
- [`../schemas/commands/command_registry_entry.schema.json`](../schemas/commands/command_registry_entry.schema.json),
  [`../fixtures/commands/seed_commands/`](../fixtures/commands/seed_commands/),
  and
  [`../artifacts/commands/command_registry_seed.yaml`](../artifacts/commands/command_registry_seed.yaml)
  — canonical command-registry seed for aliases, discoverability
  projections, current-shortcut display, disabled-state explainers,
  diagnostics, and machine-facing names.

## Planning

- [`planning/m1_m2_dependency_backlog.md`](./planning/m1_m2_dependency_backlog.md)
  — dependency-aware and commitment-classed M1/M2 backlog grounded in
  the current M0 ADRs, prototypes, corpora, and decision gates. The
  machine-readable companions live in
  [`/artifacts/planning/`](../artifacts/planning/).
  Program-blocking dependency ids themselves live in
  [`/artifacts/governance/dependency_ledger.yaml`](../artifacts/governance/dependency_ledger.yaml).
  Commitment-class policy itself lives in
  [`/docs/governance/commitment_and_rebaseline_policy.md`](./governance/commitment_and_rebaseline_policy.md)
  and
  [`/artifacts/governance/commitment_classes.yaml`](../artifacts/governance/commitment_classes.yaml).

## Product boundary

- [`product/boundary_manifest_strawman.md`](./product/boundary_manifest_strawman.md)
  — strawman that classifies every product capability against the
  open-source core versus managed / commercial / service-plane
  boundary. Reserves deployment-profile, residual-dependency,
  data-boundary, portability, and local-core-continuity slots per
  row so later claim packets do not retrofit them inconsistently.
  Conforms to
  [`/schemas/product/boundary_manifest.schema.json`](../schemas/product/boundary_manifest.schema.json).
- [`product/onboarding_measurement_plan.md`](./product/onboarding_measurement_plan.md)
  — switching-success measurement plan covering first-run, first
  open, first useful edit, migration review, restore success, and
  opt-in-versus-continue-local behaviour. Freezes the entry-route
  taxonomy (`er.start_center`, `er.recent_work_reopen`,
  `er.restore_prompt`, `er.protocol_handler_reentry`,
  `er.clone_or_import`, `er.plain_open`, `er.workspace_switch`,
  `er.warm_start`), the readiness bucket set (`blocking_now`,
  `recommended_soon`, `optional_later`), the archetype-detection
  outcome set, and the per-surface success criterion / closed
  failure-category / event-name / derived-metric / owning-lane
  map. Seed task-success corpus in
  [`/artifacts/product/task_success_corpus_seed.yaml`](../artifacts/product/task_success_corpus_seed.yaml);
  seed no-account switching scoreboard in
  [`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../artifacts/product/no_account_switching_scoreboard_seed.yaml).

## Deployment continuity

- [`deployment/drill_catalog_seed.md`](./deployment/drill_catalog_seed.md)
  — shared continuity, disaster-recovery, mirror/offline, and
  control-plane/data-plane impairment drill catalog seed used by
  release planning, support/export, and boundary-manifest planning.
  Machine-readable seed in
  [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../artifacts/support/deployment_drill_catalog_seed.yaml);
  concrete impairment fixtures in
  [`/fixtures/deployment/impairment_cases/`](../fixtures/deployment/impairment_cases/).

## Supportability

- [`support/support_center_concept.md`](./support/support_center_concept.md)
  — concept note for Project Doctor, safe mode, support-bundle preview,
  repair-preview transactions, object-specific issue handoff, and the
  exact-build/docs/route truth the support center should preserve.
  Machine-readable support packet family index in
  [`/schemas/support/support_packet_index.schema.json`](../schemas/support/support_packet_index.schema.json).
- [`support/support_bundle_contract.md`](./support/support_bundle_contract.md)
  — governed support/export bundle packet with exact-build, route,
  continuity, recovery-ladder, fault-domain, consent, waiver, and
  typed artifact-manifest semantics. Machine-readable schema in
  [`/schemas/support/support_bundle.schema.json`](../schemas/support/support_bundle.schema.json);
  seed redaction profiles and example bundles in
  [`/fixtures/support/`](../fixtures/support/).
- [`support/exact_build_symbolication_smoke.md`](./support/exact_build_symbolication_smoke.md)
  — local crash-to-symbolication smoke path proving one exact-build
  identity can drive native symbols, renderer source maps, crash dump
  manifests, support-bundle references, and fail-closed mismatch
  handling. Fixture corpus in
  [`/fixtures/support/crash_fixture/`](../fixtures/support/crash_fixture/);
  retention/redaction seed in
  [`/artifacts/support/crash_artifact_retention_seed.json`](../artifacts/support/crash_artifact_retention_seed.json);
  runner in
  [`/tools/support/symbolicate_smoke.sh`](../tools/support/symbolicate_smoke.sh).

## Frozen vocabularies

- [`docs/help_about_service_health_routes.md`](./docs/help_about_service_health_routes.md)
  — shared destination-descriptor contract for Help, About,
  service-health, docs-browser, migration, onboarding, provenance,
  community-handoff, and support-export routes. Freezes the
  product-bound field set for destination trust/owner/boundary,
  source/version, exact-build applicability, support class,
  client scopes, freshness, availability, locale/offline posture,
  route class, browser/device-code rules, issue-template support,
  and data-exit boundary. Boundary schema in
  [`/schemas/docs/destination_descriptor.schema.json`](../schemas/docs/destination_descriptor.schema.json);
  worked seed descriptors in
  [`/artifacts/docs/destination_descriptor_seed.yaml`](../artifacts/docs/destination_descriptor_seed.yaml).
- [`docs/docs_pack_manifest_contract.md`](./docs/docs_pack_manifest_contract.md)
  — docs-pack manifest contract consumed by every docs pane, docs
  browser, Help / About footer, service-health row, support summary,
  onboarding step, and AI-explanation overlay that resolves a
  `help_status_badge_record.source_revision_ref`. Freezes the closed
  source-class set admissible on a manifest (`project_docs`,
  `generated_reference`, `mirrored_official_docs`,
  `curated_knowledge_pack`, `support_runbook`), publisher classes,
  signature-status states, mirror-chain continuity rules,
  acquired-via values (including `air_gapped_media` with an
  `offline_expiration_at` deadline), per-locale coverage classes
  (`complete`, `partial`, `machine_assisted`, `stub`, `stale_copy`),
  example-label vocabulary (`stable_example`, `stale_example`,
  `needs_review_example`, `quarantined_example`) and stale-example
  reasons, citation / backlink postures, publishable-state gate,
  and the closed set of publishable-blocking reasons. Boundary
  schema in
  [`/schemas/docs/docs_pack_manifest.schema.json`](../schemas/docs/docs_pack_manifest.schema.json);
  worked fixtures (fresh, offline / mirrored, partially stale,
  mixed-locale, newer-than-client, non-publishable, and a standalone
  stale-example record) in
  [`/fixtures/docs/docs_pack_examples/`](../fixtures/docs/docs_pack_examples/).
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
- [`workspace/layout_serialization_contract.md`](./workspace/layout_serialization_contract.md)
  — workspace-layout serialization boundary covering workspace
  authority versus window topology, portable profile defaults,
  machine/display hints, versioned pane trees, explicit restore phases,
  missing-surface placeholders, and no-rerun guarantees for live
  surfaces. Boundary schema in
  [`/schemas/workspace/pane_tree.schema.json`](../schemas/workspace/pane_tree.schema.json);
  worked fixtures in
  [`/fixtures/workspace/layout_serialization_examples/`](../fixtures/workspace/layout_serialization_examples/).
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
- [`security/safe_preview_trust_classes.md`](./security/safe_preview_trust_classes.md)
  — safe-preview trust-class and suspicious-content vocabulary for raw
  text, sanitized rich content, trusted local active content, and
  isolated remote / embedded content. Freezes owner/origin chrome,
  downgrade-to-static-snapshot or metadata-only behavior, strict
  annotation rules on install/publish/attach/approval/delete-review
  surfaces, and the explicit transfer actions `copy_raw`,
  `copy_rendered`, `copy_escaped`, `export_sanitized_snapshot`, and
  `export_metadata_only`. Boundary schemas in
  [`/schemas/security/trust_class.schema.json`](../schemas/security/trust_class.schema.json)
  and
  [`/schemas/security/text_representation_policy.schema.json`](../schemas/security/text_representation_policy.schema.json);
  worked fixtures in
  [`/fixtures/security/suspicious_content_cases/`](../fixtures/security/suspicious_content_cases/).
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
- [`state/migration_and_restore_playbook.md`](./state/migration_and_restore_playbook.md)
  — shared migration, downgrade, and restore-provenance playbook for
  profile, layout, sync, checkpoint, support-export, and repair
  surfaces. Freezes the four state planes (`portable_settings`,
  `local_context`, `workspace_shared_manifest`,
  `non_portable_live_authority`), the shared fidelity-label set
  (`exact`, `compatible`, `layout_only`, `manual_review`), typed
  downgrade and failure-state rows, and the rule that prior artifacts
  stay preserved for compare/export when schema meaning changed or a
  restore stopped short of faithful apply. Boundary schema in
  [`/schemas/state/restore_provenance.schema.json`](../schemas/state/restore_provenance.schema.json);
  worked fixtures covering all four fidelity labels in
  [`/fixtures/state/migration_cases/`](../fixtures/state/migration_cases/).
- [`state/profile_and_state_map.md`](./state/profile_and_state_map.md)
  — portable-profile artifact and configuration / state map seed
  every profile export / import, managed-sync lane, support-bundle
  exporter, restore / migration surface, and remembered-state
  inspector resolves against. Freezes four record kinds
  (`portable_profile_artifact_record`, `state_map_row_record`,
  `export_manifest_record`, `restore_provenance_record`), the four
  state-authority classes (user-authored durable truth, user-owned
  recovery state, admin / control artifact, disposable derived
  cache), the six portability classes, the six profile modes, the
  four restore-fidelity labels (`exact`, `compatible`, `layout_only`,
  `manual_review`), the eight location-root ids, and the
  Appendix-F-style state map that pins authority, portability,
  retention, sync posture, support-export posture, clear posture,
  and redaction per state class. Reserves the remembered-state
  inspector's `stable_pane_id`, `state_classes_exposed`, and
  export / clear / compare action slots. Boundary schema in
  [`/schemas/profile/portable_profile.schema.json`](../schemas/profile/portable_profile.schema.json);
  worked fixtures (plain portable profile, paired export manifest,
  state-map row for the execution-context cache, and one restore-
  provenance fixture per fidelity label) in
  [`/fixtures/profile/restore_provenance_examples/`](../fixtures/profile/restore_provenance_examples/).
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
- [`migration/migration_center_object_model.md`](./migration/migration_center_object_model.md)
  — durable migration-center session, importer-outcome, shortcut-digest,
  and restore-record contract for import flows that must stay
  reviewable after first-run or entry-surface UI closes. Freezes the
  session fields for source tool/version, selected domains, target
  profile/workspace, actor, restore-checkpoint linkage, and
  compatibility-report linkage; the controlled importer outcome
  vocabulary (`imported`, `mapped`, `skipped`, `manual_review`,
  `bridge_required`, `unsupported`); and machine-readable report,
  issue-template, support, and export refs shared across docs and
  support handoff. Boundary schemas in
  [`/schemas/migration/migration_session.schema.json`](../schemas/migration/migration_session.schema.json)
  and
  [`/schemas/migration/importer_outcome.schema.json`](../schemas/migration/importer_outcome.schema.json);
  both schemas embed worked examples.
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
- [`runtime/resource_governor_contract.md`](./runtime/resource_governor_contract.md)
  — shared runtime resource-governor contract covering protected
  work classes, threshold families, queue and shed order,
  admission-control rules, and visible health-state semantics for
  `ready`, `warming`, `partial`, `degraded`, `offline`,
  `unsupported`, and `overloaded`. Machine-readable thresholds,
  policy fixtures, and overload scenarios live in
  [`/artifacts/runtime/resource_governor_thresholds.yaml`](../artifacts/runtime/resource_governor_thresholds.yaml).
- [`perf/efficiency_state_policy.md`](./perf/efficiency_state_policy.md)
  — battery, thermal, and power-saver specialization of the shared
  runtime governor. Freezes the efficiency-state model, worker-
  budget governance for AI warmups / prefetch / uploads /
  indexing / extension polling / previews / graph enrichment,
  hidden-pane and off-screen render suppression, and the visible
  cue requirements that accompany partial or stale results under
  constrained modes. Machine-readable rules live in
  [`/artifacts/perf/worker_budget_rules.yaml`](../artifacts/perf/worker_budget_rules.yaml);
  reviewable suppression scenarios live in
  [`/fixtures/perf/hidden_pane_cases/`](../fixtures/perf/hidden_pane_cases/).
- [`perf/power_thermal_methodology.md`](./perf/power_thermal_methodology.md)
  — reproducible raw-capture and audit methodology for laptop power,
  battery-drain, thermal-transition, hidden-pane suppression, and
  worker-budget claims. Freezes the reference-laptop matrix, raw
  capture schema, measurement windows, and audit-script contract so
  power / thermal recalibration lands as an explicit methodology
  change instead of piggybacking on feature work. Machine-readable
  reference profiles live in
  [`/artifacts/perf/reference_laptop_matrix.yaml`](../artifacts/perf/reference_laptop_matrix.yaml);
  benchmark hardware rows and display classes live in
  [`/artifacts/perf/reference_hardware_manifest.yaml`](../artifacts/perf/reference_hardware_manifest.yaml);
  lab-image revisions and calibration rules live in
  [`/artifacts/perf/lab_image_manifest.yaml`](../artifacts/perf/lab_image_manifest.yaml);
  raw capture schema lives in
  [`/schemas/benchmarks/power_thermal_capture.schema.json`](../schemas/benchmarks/power_thermal_capture.schema.json);
  example captures live in
  [`/fixtures/perf/power_thermal_capture_examples/`](../fixtures/perf/power_thermal_capture_examples/).
- [`perf/self_capture_parity.md`](./perf/self_capture_parity.md)
  — guidance for comparing local-machine `self_capture` runs against
  reference rows without pretending they are identical. Reuses the same
  hardware rows, environment rows, display classes, and lab-image
  revisions the benchmark dashboard and publication packet now cite.
- [`commands/command_descriptor_contract.md`](./commands/command_descriptor_contract.md)
  — command-descriptor contract every palette, application /
  context menu, keybinding / shortcut-help layer, CLI help, AI-
  tool surface, automation recipe, and invocation-session packet
  reads before a command is surfaced, enabled, disabled with a
  typed reason, previewed, approved, executed, or replayed.
  Freezes the `command_descriptor_record` (stable command id,
  canonical verb, accessibility label path, docs / help anchor,
  shortcut narration, typed arguments, capability scope class,
  discoverability metadata, UI-slot hints, lifecycle metadata,
  result contract) and the `invocation_session_packet_record`
  (issuing surface, authority class, argument-provenance map,
  context snapshot, enablement decision, preview / approval
  posture, execution intent, outcome, created-artifact refs,
  evidence refs). Freezes the high-risk preview-class taxonomy
  (destructive, broad-scope, irreversible-publish, externally-
  mutating, credential / policy / managed / remote-attach /
  install / collaboration / browser-handoff / rich-active-content
  / bidi / confusable preview classes) and the closed disabled-
  reason vocabulary. Boundary schema in
  [`/schemas/commands/command_descriptor.schema.json`](../schemas/commands/command_descriptor.schema.json);
  worked fixtures (six first-party descriptors covering baseline
  reversible, reversible-editor, read-only-search, externally-
  publishing, policy-authoring, and destructive-snapshot-reset
  commands, plus invocation-session packets covering result-
  evidence success, approval-required pending, and disabled-with-
  reason trust denial) in
  [`/fixtures/commands/command_descriptor_examples/`](../fixtures/commands/command_descriptor_examples/).
- [`commands/command_graph_and_ui_slots_seed.md`](./commands/command_graph_and_ui_slots_seed.md)
  — slot-taxonomy seed translating the command-descriptor
  contract's coarse `ui_slot_hints` into stable shell/help/
  onboarding/companion slot families and slot keys, plus one
  slot-token publication strategy that maps slot families to the
  design-token export's family and state vocabularies without
  changing either upstream schema. Boundary schema in
  [`/schemas/commands/ui_slot_taxonomy.schema.json`](../schemas/commands/ui_slot_taxonomy.schema.json);
  worked fixtures (one taxonomy seed record and one cross-surface
  command projection example) in
  [`/fixtures/commands/ui_slot_taxonomy_examples/`](../fixtures/commands/ui_slot_taxonomy_examples/).
- [`commands/command_parity_diff.md`](./commands/command_parity_diff.md)
  — reusable cross-surface parity diff format plus the current
  seed report for launch-bearing command surfaces. Compares stable
  command id, label/alias, enablement rules, preview posture,
  authority class, and result contract across palette,
  menu/button, keybinding-help, CLI/help, and AI-tool claims.
  Seed corpus in
  [`/artifacts/commands/command_parity_seed.yaml`](../artifacts/commands/command_parity_seed.yaml);
  generator in
  [`/tools/commands/parity_diff_seed.py`](../tools/commands/parity_diff_seed.py).
- [`design/design_token_component_state_vocabulary.md`](./design/design_token_component_state_vocabulary.md)
  — design-token, component-state, theme-support, accessibility-
  posture, and layer / scrim vocabulary every shell, docs / help,
  trust, onboarding, and durable-attention surface consumes from
  M0 onward. Freezes the token-family set (color_brand,
  color_functional_accent, color_neutral, color_state,
  color_semantic_theme, color_syntax, color_diff, color_chart,
  typography_role, typography_scale, text_rule, spacing, sizing,
  radius, border_stroke, elevation, opacity_scrim,
  layer_portal_order, motion_duration, motion_easing,
  motion_restriction, density, icon_treatment, semantic_status,
  trust_visual_state), the component-state set (idle, hover,
  focus, focus_visible, pressed, selected, disabled, loading,
  pending, degraded, stale, restricted, policy_blocked, warning,
  destructive, reconnecting, completed, restored,
  quiet_hours_held), the theme set (dark_reference, light_parity,
  high_contrast_dark, high_contrast_light), the accessibility
  posture set (motion_standard, motion_reduced, motion_low_motion,
  motion_power_saver, motion_critical_hot_path), the layer /
  portal order set (z_base, z_sticky, z_floating, z_menu,
  z_dialog, z_toast, z_critical), and the scrim / overlay set
  (scrim_none, scrim_weak, scrim_strong,
  overlay_dim_presentation). Boundary schema in
  [`/schemas/design/token_export_manifest.schema.json`](../schemas/design/token_export_manifest.schema.json);
  theme and accessibility-posture rows in
  [`/artifacts/design/theme_support_rows.yaml`](../artifacts/design/theme_support_rows.yaml);
  layer and scrim tokens in
  [`/artifacts/design/layer_and_scrim_tokens.yaml`](../artifacts/design/layer_and_scrim_tokens.yaml).

## Benchmarks and corpus

- [`benchmarks/corpus_governance.md`](./benchmarks/corpus_governance.md)
  — benchmark corpus-governance and protected-metric change-control
  policy. Freezes the governance asset matrix, change classes,
  approval paths, PR-separation rules, external/customer-derived corpus
  rules, and the changelog requirements for threshold easing or
  protected-path corpus removal. Machine-readable companions in
  [`/artifacts/bench/corpus_change_control.yaml`](../artifacts/bench/corpus_change_control.yaml)
  and
  [`/artifacts/bench/protected_metrics.yaml`](../artifacts/bench/protected_metrics.yaml).
- [`benchmarks/fitness_function_catalog.md`](./benchmarks/fitness_function_catalog.md)
  — normative companion to the protected fitness-function catalog.
  Freezes the closed vocabularies for row status, architecture
  driver (the nine TAD §3.2 quality-attribute drivers), architecture
  principle (the ten TAD §4.1 principles), protected journey
  (aligned with spike-metric-names and the corpus manifest),
  protected SLO family, threshold mode, data-source kind, waiver
  authority, and review cadence. Names the seven seeded rows (warm
  start, first paint, input-to-paint, buffer operations, VFS save /
  conflict handling, benchmark-lab operational health) and three
  provisional rows (power / thermal posture, restore fidelity,
  command-graph parity). Pins waiver authority to
  `performance_council` plus the lane DRI per
  [`governance/dri_map.md`](./governance/dri_map.md) §Authority, and
  pins the `fitness_function_snapshot` export shape every shiproom
  and benchmark-report packet consumes. Machine-readable register in
  [`/artifacts/bench/fitness_function_catalog.yaml`](../artifacts/bench/fitness_function_catalog.yaml).
- [`benchmarks/spike_metric_names.md`](./benchmarks/spike_metric_names.md)
  — mapping from the ADR-0002 protected-hot-path hook vocabulary to
  the journey-budget buckets the benchmark lab and journey harness
  measure against.
- [`benchmarks/benchmark_lab_run_results.md`](./benchmarks/benchmark_lab_run_results.md)
  — normative companion to the benchmark-lab run-result schema and
  the seeded dashboard baseline. Freezes the run-context vocabulary
  (`reference_capture`, `provisional_capture`, `self_capture`,
  `smoke_subset`), the comparability vocabulary
  (`comparable_to_baseline`,
  `comparable_to_prior_run_same_host`, `not_yet_comparable`,
  `quarantined`), the quarantine-reason set, the row-result /
  trend-direction / threshold-mode / SLI-kind / data-source-kind /
  lane-class / trigger-kind sets, and the nine
  `regression_trigger_ref.kind` values every fail or warn row
  resolves against (`threshold_exceeded`, `boolean_gate_failed`,
  `ratio_below_floor`, `corpus_row_missing`,
  `trace_schema_nonconforming`, `ad_hoc_metric_name_observed`,
  `toolchain_pin_drift`, `hardware_definition_mismatch`,
  `fitness_catalog_row_status_provisional`). Pins the
  reproducibility posture (`SOURCE_DATE_EPOCH`, `TZ=UTC`,
  `LC_ALL=C`) and the verify-seed gate that the nightly lane runs
  before the lab does any benchmark work. Boundary schema in
  [`/schemas/benchmarks/run_result.schema.json`](../schemas/benchmarks/run_result.schema.json);
  seeded dashboard baseline (two reference run records plus the
  rolled-up dashboard.json) in
  [`/artifacts/benchmarks/dashboard_seed/`](../artifacts/benchmarks/dashboard_seed/);
  nightly CI lane in
  [`/.github/workflows/nightly_benchmark.yml`](../.github/workflows/nightly_benchmark.yml).
- [`benchmarks/fixture_classes.md`](./benchmarks/fixture_classes.md)
  — normative vocabulary for the protected benchmark corpus:
  corpus classes (`microbenchmark_scenario`, `workflow_scenario`,
  `archetype_seed`, `large_file_trigger`,
  `recovery_or_restore_scenario`, `reference_workspace`,
  `boundary_truth_case`), size classes, visibility / retention /
  license classes, host-platform and toolchain postures, archetype
  placeholder tags, support classes, evidence-consumer channels,
  `resolution_mode` rules (`concrete_file`, `live_repo_slice`,
  `recipe_only`), and segregation-marker rules. Companion to the
  machine-readable manifest in
  [`/fixtures/benchmarks/corpus_manifest.yaml`](../fixtures/benchmarks/corpus_manifest.yaml);
  reference-workspace seeds live under
  [`/fixtures/workspaces/reference/`](../fixtures/workspaces/reference/).
- [`benchmarks/journey_trace_taxonomy.md`](./benchmarks/journey_trace_taxonomy.md)
  — normative companion to the protected user-journey trace record.
  Freezes the closed vocabularies for `journey_class`
  (`startup_to_first_useful_chrome`, `startup_to_first_paint`,
  `shell_open`, `placeholder_open`, `placeholder_edit`,
  `placeholder_save`, `open_edit_save`, `restore_adjacent`,
  `recovery_journal_restore_flow`, `boundary_truth_contract_replay`),
  `checkpoint_class` (`journey_start`, `journey_end`,
  `protected_path_event`, `degraded_transition`,
  `fallback_transition`, `provisional_segment_boundary`),
  `segment_class` (every `protected_journey_class` value plus
  `provisional_segment`), `degraded_posture_class` (`healthy`,
  `reduced_chrome_only`, `degraded_renderer_banner_visible`,
  `responsive_fallback_active`,
  `missing_target_recovered_to_layout_only`,
  `missing_target_recovered_to_compatible`), and
  `fallback_posture_class` (`none`, `glyph_fallback_active`,
  `atlas_shard_rebind`, `atlas_eviction_observed`,
  `software_renderer_active`, `recovery_journal_replay_active`).
  Pins the reserved nullable `hardware_definition_ref`,
  `environment_ref`, `exact_build_identity_ref`,
  `linked_spike_trace_refs`, `evidence_refs`, and
  `requirement_refs` slots so reference-capture, release-evidence,
  and protected-path budget stitching can attach without a schema
  bump.
- [`benchmarks/protected_path_ledgers.md`](./benchmarks/protected_path_ledgers.md)
  — normative companion to the protected-path, latency-budget, and
  evidence-linkage ledgers. Freezes the stable path ids
  (`path.shell.launch`, `path.shell.first_useful_chrome`,
  `path.command_palette.open`, `path.editor.placeholder_open`,
  `path.editor.first_useful_edit`, `path.editor.save`,
  `path.workspace.restore`,
  `path.onboarding.start_center_first_useful_edit`), the stable versus
  provisional segment-status vocabulary, the budget-source-kind set
  (`published_ux_budget`, `protected_metrics_contract`,
  `provisional_engineering_target`,
  `degraded_state_fallback_rule`), the packet families that review a
  path, and the named-change-record rule for path additions or
  removals. Machine-readable companions in
  [`/artifacts/perf/protected_path_ledger.yaml`](../artifacts/perf/protected_path_ledger.yaml),
  [`/artifacts/perf/latency_budget_ledger.yaml`](../artifacts/perf/latency_budget_ledger.yaml),
  and
  [`/artifacts/perf/evidence_linkage_seed.yaml`](../artifacts/perf/evidence_linkage_seed.yaml).
  and requirement-linkage lanes attach without a schema version
  bump. Boundary schema in
  [`/schemas/traces/journey_trace.schema.json`](../schemas/traces/journey_trace.schema.json);
  committed seeds for startup-to-first-useful-chrome,
  open-edit-save, and restore-adjacent journeys in
  [`/fixtures/journeys/`](../fixtures/journeys/); harness wrapper
  at [`/tools/journey_harness.sh`](../tools/journey_harness.sh)
  and stdlib-only emitter at
  [`/tools/journey_harness/journey_harness.py`](../tools/journey_harness/journey_harness.py).

## Release engineering

- [`release/install_topology_plan.md`](./release/install_topology_plan.md)
  — pre-implementation plan for install topology, per-channel
  separation rules, fleet-ring promotion ladder, and state-root
  mapping. Freezes the `install_profile_card_record` shape every
  release, fleet, About / Help, support-bundle, and diagnostics
  surface reads, plus closed vocabularies for install mode, channel,
  updater owner, binary root, durable state root, side-by-side
  relation, rollback target, diagnostics export, rollout ring
  (`canary`, `pilot`, `broad`, `lts`), silent-install support,
  managed-package report, publication posture (`online_vendor`,
  `offline_signed_bundle`, `customer_managed_mirror`,
  `third_party_package_index`), policy injection, and return-code
  family. Pins per-channel separation rules so no side-by-side row
  silently corrupts another row's state markers, and pins the
  minimum promotion evidence plus admissible rollback target per
  ring. Companion artifacts:
  [`/artifacts/release/install_topology_matrix.yaml`](../artifacts/release/install_topology_matrix.yaml)
  (one install-profile card per
  `(install_mode_class, channel_class, platform_class,
  architecture_class)` tuple with rules and ring-promotion
  evidence),
  [`/artifacts/release/state_root_map.yaml`](../artifacts/release/state_root_map.yaml)
  (one state-root row per `(durable_state_root_class,
  channel_class)` pair with owning_channels, authority class,
  scriptability, diagnostics visibility, repair / verify, and
  exact-build install diagnostic classes), and
  [`/artifacts/release/silent_deployment_seed.yaml`](../artifacts/release/silent_deployment_seed.yaml)
  (return-code families aligned with the stable CLI exit-code
  model, the `unattended_deployment_result_record` shape, and ten
  worked fixtures covering install success, update partial-success
  with reboot required, managed trust-policy denial, air-gap
  mirror metadata stale, update rollback_required plus its
  rollback follow-up, verify-failed, managed uninstall
  admin_required, portable spill detected, and channel-pin
  success). No installer, updater, or fleet tooling is implemented
  at this milestone; the plan is the vocabulary and row-shape
  layer every later release-engineering, continuity, fleet-
  rollout, desktop-affordance, and endpoint-posture lane composes
  over.

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
- [`/artifacts/governance/experiments_register.yaml`](../artifacts/governance/experiments_register.yaml)
  — canonical control register for experiments, feature flags,
  benchmark modes, hidden developer toggles, rollout rows, and
  reserved control-stack bindings. Every row carries owner,
  lifecycle, review or expiry, provider-chain disclosure, offline
  posture, kill switch, rollback path, and artifact dependencies.
- [`/artifacts/governance/labs_register.yaml`](../artifacts/governance/labs_register.yaml)
  — contributor-visible Labs / prototype / preview inventory
  projected from `experiments_register.yaml`, intentionally
  excluding hidden developer toggles.
- [`/artifacts/governance/decision_index.yaml`](../artifacts/governance/decision_index.yaml)
  — decision rows with freeze dates and default-if-unresolved postures.
- [`/artifacts/governance/package_inventory.yaml`](../artifacts/governance/package_inventory.yaml)
  — package topology and protected-path posture.
- [`/artifacts/governance/dependency_register.yaml`](../artifacts/governance/dependency_register.yaml)
  — canonical register of selected and admitted third-party
  dependencies.
- [`/artifacts/governance/third_party_import_register.yaml`](../artifacts/governance/third_party_import_register.yaml)
  — canonical register of copied, bundled, or mirrored third-party
  bytes.
- [`/artifacts/governance/release_notice_seed.yaml`](../artifacts/governance/release_notice_seed.yaml)
  — third-party attribution seed keyed by stable dependency/import ids.
- [`/artifacts/governance/compliance_checklist.yaml`](../artifacts/governance/compliance_checklist.yaml)
  — bridge artifact pointing at the canonical dependency/import/notice
  registers, plus deferred compliance sweeps.
- [`/artifacts/governance/milestone_scorecard_template.yaml`](../artifacts/governance/milestone_scorecard_template.yaml)
  — per-milestone lane status.
- [`/artifacts/governance/governance_packet_template.yaml`](../artifacts/governance/governance_packet_template.yaml)
  — verification, benchmark-report, compatibility-report, claim-manifest,
  shiproom, and waiver-register packet families.
- [`/schemas/governance/`](../schemas/governance/) — schemas the YAML
  registers conform to.
- [`/schemas/product/boundary_manifest.schema.json`](../schemas/product/boundary_manifest.schema.json)
  — contract for the product boundary manifest strawman.
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../fixtures/benchmarks/corpus_manifest.yaml)
  — protected benchmark corpus manifest: one register of every
  fixture the benchmark lab, journey harness, boundary-truth
  validators, compatibility scoreboards, and support-export lanes
  read, with corpus class, size class, visibility / retention /
  license status, host-platform and toolchain posture, protected
  journeys exercised, archetype tags, support classes, evidence-
  consumer channels, source lineage, and segregation markers per
  fixture. Normative companion in
  [`/docs/benchmarks/fixture_classes.md`](./benchmarks/fixture_classes.md).
- [`/artifacts/bench/corpus_change_control.yaml`](../artifacts/bench/corpus_change_control.yaml)
  — machine-readable benchmark-governance change-control register:
  governance asset matrix, change classes, CI gates, external/customer
  corpus admission rules, PR-separation rule, and protected-path
  changelog template.
- [`/artifacts/bench/protected_metrics.yaml`](../artifacts/bench/protected_metrics.yaml)
  — revisioned protected-metrics file: threshold snapshot, rationale,
  comparability note, calibration state/date, change authority, CI
  review rule, and public-reporting posture for every protected
  fitness row.
- [`/artifacts/perf/reference_hardware_manifest.yaml`](../artifacts/perf/reference_hardware_manifest.yaml)
  — canonical benchmark hardware-row and display-class register. Every
  benchmark packet now cites one hardware row from this manifest rather
  than a free-text machine label.
- [`/artifacts/perf/lab_image_manifest.yaml`](../artifacts/perf/lab_image_manifest.yaml)
  — lab-image revision, benchmark-environment row, display/power/
  thermal posture, and calibration-checklist register shared by
  benchmark packets, the dashboard seed, and self-capture parity docs.
- [`/artifacts/perf/protected_path_ledger.yaml`](../artifacts/perf/protected_path_ledger.yaml)
  — stable protected-path register: path ids, owners, measurement
  boundaries, stable and provisional segment ids, budget-row refs,
  evidence-row refs, and append-only named change records for path
  additions or removals.
- [`/artifacts/perf/latency_budget_ledger.yaml`](../artifacts/perf/latency_budget_ledger.yaml)
  — per-path budget sheet: threshold provenance, threshold values,
  measurement sources, fixture/reference-workspace refs, fail-soft
  posture, and waiver authority for every protected path row.
- [`/artifacts/perf/evidence_linkage_seed.yaml`](../artifacts/perf/evidence_linkage_seed.yaml)
  — per-path evidence joins: journey traces, trace-segment refs,
  benchmark-corpus refs, task-success scenario refs, qualification-row
  refs, packet families, and the continuity/local-history hooks the
  save and restore rows reserve.
- [`/schemas/benchmarks/run_result.schema.json`](../schemas/benchmarks/run_result.schema.json)
  — boundary schema for one benchmark-lab run-result record. Pins
  every record to a single exact-build identity, a single corpus-
  manifest revision, a single protected-metrics revision, a single
  fitness-catalog revision, a single hardware-definition ref, and a
  single benchmark-environment ref; freezes closed vocabularies for run
  context, comparability, quarantine reason, row result, trend
  direction, threshold mode, SLI kind, data-source kind, lane /
  trigger class, and
  `regression_trigger_ref.kind`. Normative
  companion in
  [`/docs/benchmarks/benchmark_lab_run_results.md`](./benchmarks/benchmark_lab_run_results.md);
  seeded dashboard baseline in
  [`/artifacts/benchmarks/dashboard_seed/`](../artifacts/benchmarks/dashboard_seed/).
- [`/artifacts/bench/fitness_function_catalog.yaml`](../artifacts/bench/fitness_function_catalog.yaml)
  — protected fitness-function catalog: one register of every
  protected fitness function the benchmark lab, journey harness,
  release-evidence shiproom packets, and performance-council
  waiver log resolve against by stable id. Carries closed
  vocabularies for architecture driver / principle / protected
  journey / protected SLO family / threshold mode / data-source
  kind / waiver authority / review cadence, a
  `packet_export_shape.fitness_function_snapshot` block consumed
  by benchmark-report and shiproom packets, and a `slices:` block
  indexed by driver, principle, journey, waiver authority, and
  SLO family. Normative companion in
  [`/docs/benchmarks/fitness_function_catalog.md`](./benchmarks/fitness_function_catalog.md).
- [`/schemas/traces/journey_trace.schema.json`](../schemas/traces/journey_trace.schema.json)
  — boundary schema for one protected user-journey trace record
  the journey harness (tools/journey_harness) emits for startup,
  shell open, placeholder file open / edit / save, and
  restore-adjacent flows. Pins every record to one fixture id, one
  corpus-manifest revision, one minimum build-identity record, one
  `degraded_posture`, and one `fallback_posture`, with reserved
  nullable slots for `hardware_definition_ref`, `environment_ref`,
  `exact_build_identity_ref`, `linked_spike_trace_refs`,
  `evidence_refs`, and `requirement_refs`. Seeded traces in
  [`/fixtures/journeys/`](../fixtures/journeys/). Normative
  companion in
  [`/docs/benchmarks/journey_trace_taxonomy.md`](./benchmarks/journey_trace_taxonomy.md).
- [`/artifacts/release/install_topology_matrix.yaml`](../artifacts/release/install_topology_matrix.yaml)
  — install-topology matrix with one install-profile card per
  `(install_mode_class, channel_class, platform_class,
  architecture_class)` tuple, plus the canary / pilot / broad /
  lts ring-promotion evidence table and the rules the matrix
  enforces (no shared durable state across channels, portable
  no machine-global mutation, no last-writer-wins for
  associations, managed_fleet requires a managed-package
  report, and rollback target must match ring). Normative
  companion in
  [`/docs/release/install_topology_plan.md`](./release/install_topology_plan.md).
- [`/artifacts/release/state_root_map.yaml`](../artifacts/release/state_root_map.yaml)
  — install-topology state-root map with one state-root row per
  `(durable_state_root_class, channel_class)` pair, plus
  update-marker, recent-item-registration, file-association, and
  protocol-handler ownership rows per channel. Pins the
  `no_shared_durable_state_across_channels` collision policy at
  the schema level so no side-by-side row silently corrupts
  another row's state markers. Normative companion in
  [`/docs/release/install_topology_plan.md`](./release/install_topology_plan.md).
- [`/artifacts/release/silent_deployment_seed.yaml`](../artifacts/release/silent_deployment_seed.yaml)
  — silent-deployment return-code family seed. Freezes the ten
  return-code families (`success`, `partial_success`,
  `user_config_error`, `trust_policy_denial`, `missing_dependency`,
  `network_transport`, `internal_failure`, `rollback_required`,
  `verification_failed`, `admin_required`) aligned with the
  stable CLI exit-code model, plus the fifteen-value
  `failure_reason_class` and the nine-value
  `remediation_pointer_class` vocabularies and the
  `unattended_deployment_result_record` shape with ten worked
  fixtures. Normative companion in
  [`/docs/release/install_topology_plan.md`](./release/install_topology_plan.md).
