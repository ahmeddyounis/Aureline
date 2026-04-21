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

- [`benchmarks/spike_metric_names.md`](./benchmarks/spike_metric_names.md)
  — mapping from the ADR-0002 protected-hot-path hook vocabulary to
  the journey-budget buckets the benchmark lab and journey harness
  measure against.
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
