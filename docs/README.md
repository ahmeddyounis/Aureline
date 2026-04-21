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
- [`governance/feature_flag_policy.md`](./governance/feature_flag_policy.md)
  — normative policy for experiments, feature flags, Labs inventory,
  rollout rows, policy disables, and kill switches before a runtime
  control plane exists.
- [`governance/decision_backlog.md`](./governance/decision_backlog.md)
  — seeded architecture decisions with freeze dates and default
  narrowing postures.
- [`governance/decision_workflow.md`](./governance/decision_workflow.md)
  — how decisions open, close, supersede, and narrow.
- [`governance/dogfood_issue_taxonomy.md`](./governance/dogfood_issue_taxonomy.md)
  — dogfood intake taxonomy covering category, severity, evidence-link,
  exact-build, route-truth, and hidden-dependency fields for issue
  templates and intake automation.
- [`governance/templates/`](./governance/templates/) — waiver,
  verification packet, and freeze-exception templates.
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
- [`build/exact_build_identity_model.md`](./build/exact_build_identity_model.md)
  — exact-build identity model and per-surface propagation rules for
  release, docs/help, benchmark, and support/export truth.
- [`build/reproducible_build_baseline.md`](./build/reproducible_build_baseline.md)
  — pinned toolchain, bootstrap command, and build-identity record.
- [`build/cleanroom_rebuild_lane.md`](./build/cleanroom_rebuild_lane.md)
  — first clean-room rebuild lane, emitted input-manifest shape,
  artifact-digest comparison rules, and named reproducibility gaps.

## Release and benchmark publication

- [`release/release_artifact_graph.md`](./release/release_artifact_graph.md)
  — publishable release-artifact graph, bundle-completeness rules, and
  contract-surface index.
- [`release/release_evidence_packet_template.md`](./release/release_evidence_packet_template.md)
  — release-truth packet template and waiver-aware shiproom structure.
- [`benchmarks/benchmark_publication_pack_template.md`](./benchmarks/benchmark_publication_pack_template.md)
  — public benchmark/public-proof packet template with exact command
  line, comparability, protected-metrics revision, docs applicability,
  exclusion, and competitor disclosure fields.
- [`benchmarks/public_comparison_rules.md`](./benchmarks/public_comparison_rules.md)
  — methodology-only versus claim-bearing publication rules and
  head-to-head comparison disclosure requirements.

## Planning

- [`planning/m1_m2_dependency_backlog.md`](./planning/m1_m2_dependency_backlog.md)
  — dependency-aware and commitment-classed M1/M2 backlog grounded in
  the current M0 ADRs, prototypes, corpora, and decision gates. The
  machine-readable companions live in
  [`/artifacts/planning/`](../artifacts/planning/).

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

## Frozen vocabularies

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
- [`/schemas/benchmarks/run_result.schema.json`](../schemas/benchmarks/run_result.schema.json)
  — boundary schema for one benchmark-lab run-result record. Pins
  every record to a single exact-build identity, a single corpus-
  manifest revision, a single protected-metrics revision, a single
  fitness-catalog revision, and a single hardware-definition ref;
  freezes closed vocabularies for run context, comparability,
  quarantine reason, row result, trend direction, threshold mode, SLI
  kind, data-source kind, lane / trigger class, and
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
