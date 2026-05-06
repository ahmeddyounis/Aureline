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
- [`governance/frozen_surface_ci_policy.md`](./governance/frozen_surface_ci_policy.md)
  — same-train CI policy for seeded frozen surfaces, including the
  manifest-driven diff metadata and companion-update rules.
- [`governance/benchmark_council_charter.md`](./governance/benchmark_council_charter.md)
  — seed charter for the benchmark council (roles, scope, cadence,
  quorum placeholder, escalation).
- [`governance/feature_flag_policy.md`](./governance/feature_flag_policy.md)
  — normative policy for experiments, feature flags, Labs inventory,
  rollout rows, policy disables, and kill switches before a runtime
  control plane exists.
- [`policy/admin_policy_and_bundle_cache_contract.md`](./policy/admin_policy_and_bundle_cache_contract.md)
  — local admin-policy artifact, signed bundle-cache, precedence,
  safe-default, and explain/export contract. Boundary schemas live in
  [`/schemas/policy/`](../schemas/policy/); worked examples live in
  [`/fixtures/policy/explain_and_diff_cases/`](../fixtures/policy/explain_and_diff_cases/).
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
- [`governance/provenance_badge_contract.md`](./governance/provenance_badge_contract.md)
  — provenance-badge, license-row, notice-row, and supply-chain-status
  contract shared by release publication, install review, dependency
  review, About / update surfaces, support exports, public-proof
  packets, and mirror/offline review. Boundary schema in
  [`/schemas/governance/provenance_badge.schema.json`](../schemas/governance/provenance_badge.schema.json);
  worked fixtures in
  [`/fixtures/governance/provenance_badge_cases/`](../fixtures/governance/provenance_badge_cases/).
- [`governance/post_install_notice_and_provenance_contract.md`](./governance/post_install_notice_and_provenance_contract.md)
  — durable post-install disclosure contract for product builds,
  installers, extensions/framework packs, mirrored or offline artifacts,
  and generated user exports. Boundary schema in
  [`/schemas/governance/post_install_disclosure.schema.json`](../schemas/governance/post_install_disclosure.schema.json);
  worked fixtures in
  [`/fixtures/governance/post_install_cases/`](../fixtures/governance/post_install_cases/).
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
  The policy-diff verification seed, chronology-bar corpus, and
  waiver-expiry dashboard contract now extend this base vocabulary in
  [`/docs/verification/policy_simulation_packet.md`](./verification/policy_simulation_packet.md),
  [`/fixtures/policy/`](../fixtures/policy/), and
  [`/artifacts/policy/`](../artifacts/policy/).
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
  boundary, claim, control-index, frozen-surface, and
  decision/source-anchor drift.
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
- [`release/release_center_object_model_contract.md`](./release/release_center_object_model_contract.md)
  — release-center object model, publish-target classes, publication
  action rows, break-glass publication rules, and UI/headless parity
  contract. Boundary schemas in
  [`/schemas/release/release_center_object.schema.json`](../schemas/release/release_center_object.schema.json)
  and
  [`/schemas/release/publish_target.schema.json`](../schemas/release/publish_target.schema.json);
  worked target fixtures in
  [`/fixtures/release/publish_target_cases/`](../fixtures/release/publish_target_cases/).
- [`release/release_status_surface_contract.md`](./release/release_status_surface_contract.md)
  — release-candidate card, version-bump row, promotion timeline,
  provenance linkage, support-window, compatibility, deprecation, and
  rollback/revocation surface contract. Boundary schemas in
  [`/schemas/release/release_candidate_card.schema.json`](../schemas/release/release_candidate_card.schema.json),
  [`/schemas/release/promotion_timeline_entry.schema.json`](../schemas/release/promotion_timeline_entry.schema.json),
  and
  [`/schemas/release/rollback_revocation_panel.schema.json`](../schemas/release/rollback_revocation_panel.schema.json);
  worked status fixtures in
  [`/fixtures/release/status_surface_cases/`](../fixtures/release/status_surface_cases/).
- [`release/update_and_rollback_contract.md`](./release/update_and_rollback_contract.md)
  — update manifest, rollback, downgrade, helper-version negotiation,
  mirror import, exact-build reconstruction, publish-target, promotion
  stage, and break-glass reconciliation contract. Boundary schemas in
  [`/schemas/release/update_manifest.schema.json`](../schemas/release/update_manifest.schema.json)
  and
  [`/schemas/release/helper_version_negotiation.schema.json`](../schemas/release/helper_version_negotiation.schema.json);
  worked flow fixtures in
  [`/fixtures/release/upgrade_downgrade_cases/`](../fixtures/release/upgrade_downgrade_cases/).
- [`release/update_ready_review_contract.md`](./release/update_ready_review_contract.md)
  — pre-apply update review, extension/package impact forecast,
  restart-required, side-by-side, emergency path, support-window risk,
  and rollback-before-restart contract. Boundary schemas in
  [`/schemas/release/update_ready_review.schema.json`](../schemas/release/update_ready_review.schema.json)
  and
  [`/schemas/release/extension_impact_forecast.schema.json`](../schemas/release/extension_impact_forecast.schema.json);
  worked review fixtures in
  [`/fixtures/release/update_ready_cases/`](../fixtures/release/update_ready_cases/).
- [`release/release_evidence_packet_template.md`](./release/release_evidence_packet_template.md)
  — release-truth packet template and waiver-aware shiproom structure.
- [`release/ring_progression_policy.md`](./release/ring_progression_policy.md)
  — validation-ring policy for widening, minimum soak expectations,
  rollback-stop defaults, evidence-reset floors, and the rule that
  stable-facing widening preserves the exact evidence snapshot behind
  the decision. Machine-readable companions in
  [`/artifacts/release/ring_matrix.yaml`](../artifacts/release/ring_matrix.yaml)
  and
  [`/schemas/release/ring_history_packet.schema.json`](../schemas/release/ring_history_packet.schema.json).
- [`release/qualification_cadence.md`](./release/qualification_cadence.md)
  — shared release-qualification plan covering cadence rows,
  rehearsal windows, proof-lane ownership, and default failure
  responses for benchmark, compatibility, support, docs, and release
  evidence.
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
- [`program/design_partner_and_public_proof_packet.md`](./program/design_partner_and_public_proof_packet.md)
  — design-partner intake, privacy-clearance, and publication-rehearsal
  packet tying external proof inputs to workflow-bundle ids, exact-build
  linkage, known limits, owners, and public-proof review steps.

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
- [`platform/deployment_and_unsupported_path_matrix.md`](./platform/deployment_and_unsupported_path_matrix.md)
  — shared disclosure matrix for tested package-manager, fleet-tool,
  helper/agent, and unsupported paths so install guides, diagnostics,
  Help, support bundles, and field triage do not infer broad platform
  claims from generic OS labels. Machine-readable companions in
  [`/artifacts/platform/tested_package_managers.yaml`](../artifacts/platform/tested_package_managers.yaml)
  and
  [`/artifacts/platform/unsupported_paths.yaml`](../artifacts/platform/unsupported_paths.yaml).
- [`ux/desktop_affordance_contract.md`](./ux/desktop_affordance_contract.md)
  — cross-surface OS affordance contract for file associations,
  open-with, reveal-in-system-shell, dock / taskbar entries,
  notification click-through, badges, system share, copied paths or
  permalinks, native dialogs, default-browser returns, deep-link review,
  and lifecycle recovery. Boundary schema in
  [`/schemas/platform/deep_link_intent.schema.json`](../schemas/platform/deep_link_intent.schema.json);
  worked fixtures in
  [`/fixtures/platform/system_affordance_cases/`](../fixtures/platform/system_affordance_cases/).
- [`ux/window_display_contract.md`](./ux/window_display_contract.md)
  — adapter-facing contract for native window controls, fullscreen,
  maximize/zoom, snapped or tiled placement, virtual desktops,
  display-topology drift, restore history, focus return, owned prompts,
  secondary windows, cross-window transfers, and presentation fallback.
  Boundary schema in
  [`/schemas/platform/window_state.schema.json`](../schemas/platform/window_state.schema.json);
  worked fixtures in
  [`/fixtures/platform/window_display_cases/`](../fixtures/platform/window_display_cases/).
- [`platform/system_open_target_truth.md`](./platform/system_open_target_truth.md)
  — system-open and native-dialog target binding contract that preserves the
  literal OS target while surfacing VFS canonical identity and alias truth
  before risky writes.
  Boundary schema in
  [`/schemas/platform/system_open_target_packet.schema.json`](../schemas/platform/system_open_target_packet.schema.json);
  worked fixtures in
  [`/fixtures/platform/network_share_alias_cases/`](../fixtures/platform/network_share_alias_cases/).
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
- [`i18n/locale_input_readiness.md`](./i18n/locale_input_readiness.md)
  — canonical locale/input readiness baseline for pseudoloc, RTL/bidi,
  CJK fallback, IME, dead keys, AltGr, emoji, fallback-chain, and
  translation-safe layout rules. Machine-readable companions in
  [`/artifacts/i18n/`](../artifacts/i18n/) and
  [`/fixtures/i18n/`](../fixtures/i18n/).
- [`accessibility/review_charter.md`](./accessibility/review_charter.md)
  — seed charter for the accessibility review lane covering owner,
  cadence, acceptance-pack families, waiver rules, and public
  backlog-label mapping. Machine-readable companions in
  [`/artifacts/accessibility/`](../artifacts/accessibility/) and
  [`/fixtures/accessibility/task_corpus_manifest.yaml`](../fixtures/accessibility/task_corpus_manifest.yaml).
- [`accessibility/visual_adaptation_contract.md`](./accessibility/visual_adaptation_contract.md)
  — high-contrast, low-saturation, color-safe diagnostic, and
  reduced-motion adaptation contract for diagnostics, diffs, charts,
  status items, badges, settings locks, screenshots, docs captures, and
  support/export evidence. Boundary schema in
  [`/schemas/ux/contrast_mode_state.schema.json`](../schemas/ux/contrast_mode_state.schema.json);
  palette artifact in
  [`/artifacts/ux/color_safe_diagnostic_palette.yaml`](../artifacts/ux/color_safe_diagnostic_palette.yaml);
  worked fixtures in
  [`/fixtures/ux/visual_adaptation_cases/`](../fixtures/ux/visual_adaptation_cases/).

## Command contracts

- [`commands/command_descriptor_contract.md`](./commands/command_descriptor_contract.md)
  — canonical command object and invocation-session packet contract for
  palette, menu, CLI/help, AI-tool, automation, and replay or audit
  surfaces.
- [`commands/command_graph_and_ui_slots_seed.md`](./commands/command_graph_and_ui_slots_seed.md)
  — slot-taxonomy and projection rules that translate descriptor
  discoverability into concrete shell slots and help surfaces.
- [`commands/sequence_and_modal_discoverability_contract.md`](./commands/sequence_and_modal_discoverability_contract.md)
  — modal-state cues, leader overlays, sequence-help rows,
  shortcut-teaching rows, and colon-style command parity as governed
  projections over the canonical command graph. Boundary schema in
  [`/schemas/commands/leader_overlay.schema.json`](../schemas/commands/leader_overlay.schema.json);
  worked fixtures in
  [`/fixtures/commands/sequence_help_examples/`](../fixtures/commands/sequence_help_examples/).
- [`ux/keybinding_resolver_contract.md`](./ux/keybinding_resolver_contract.md)
  — deterministic keybinding precedence, conflict-review,
  disabled-command explanation, import-bridge fidelity, leader
  overlay, and high-frequency shortcut-diff contract shared by
  shell, settings, migration, docs, and support surfaces. Boundary
  schema in
  [`/schemas/commands/keybinding_resolver.schema.json`](../schemas/commands/keybinding_resolver.schema.json);
  worked fixtures in
  [`/fixtures/commands/keybinding_conflict_examples/`](../fixtures/commands/keybinding_conflict_examples/).
- [`../schemas/commands/command_registry_entry.schema.json`](../schemas/commands/command_registry_entry.schema.json),
  [`../fixtures/commands/seed_commands/`](../fixtures/commands/seed_commands/),
  and
  [`../artifacts/commands/command_registry_seed.yaml`](../artifacts/commands/command_registry_seed.yaml)
  — canonical command-registry seed for aliases, discoverability
  projections, current-shortcut display, disabled-state explainers,
  diagnostics, and machine-facing names.

## Testing

- [`testing/test_item_identity_contract.md`](./testing/test_item_identity_contract.md)
  — canonical test-item identity, parameterized-case expansion,
  selector grammar, and remap/drift contract shared by editor, tree,
  CLI, watch mode, AI, imported CI, support, and release evidence.
  Boundary schemas in
  [`/schemas/testing/test_item_identity.schema.json`](../schemas/testing/test_item_identity.schema.json)
  and
  [`/schemas/testing/test_selector_grammar.schema.json`](../schemas/testing/test_selector_grammar.schema.json);
  worked fixtures in
  [`/fixtures/testing/test_item_identity_cases/`](../fixtures/testing/test_item_identity_cases/).
- [`testing/test_session_and_attempt_contract.md`](./testing/test_session_and_attempt_contract.md)
  — test session, attempt, watch-state, imported-CI projection, and
  surface reconstruction contract preserving exact selector,
  target/environment, source, raw-event, artifact, and time lineage
  across local, watched, rerun, debug-from-test, imported provider,
  support, and release evidence flows. Boundary schemas in
  [`/schemas/testing/test_session.schema.json`](../schemas/testing/test_session.schema.json)
  and
  [`/schemas/testing/test_attempt.schema.json`](../schemas/testing/test_attempt.schema.json);
  worked fixtures in
  [`/fixtures/testing/test_session_cases/`](../fixtures/testing/test_session_cases/).
- [`testing/test_quarantine_and_mute_contract.md`](./testing/test_quarantine_and_mute_contract.md)
  — release-facing quarantine, mute-state, owner/expiry, allowed
  surface, review cadence, unblock, and packet-treatment contract that
  keeps muted or quarantined test debt visible in scorecards, claim
  manifests, stable-promotion packets, and release evidence. Boundary
  schema in
  [`/schemas/testing/quarantine_record.schema.json`](../schemas/testing/quarantine_record.schema.json);
  worked fixtures in
  [`/fixtures/testing/quarantine_cases/`](../fixtures/testing/quarantine_cases/);
  policy rows in
  [`/artifacts/testing/quarantine_policy_rows.yaml`](../artifacts/testing/quarantine_policy_rows.yaml).

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

## Managed services

- [`managed/metering_and_usage_export_contract.md`](./managed/metering_and_usage_export_contract.md)
  — managed metering, quota-state, stale-meter disclosure, spend-
  attribution, downgrade, and usage-export row contract for optional
  managed-service and AI usage surfaces. Boundary schemas in
  [`/schemas/managed/quota_state.schema.json`](../schemas/managed/quota_state.schema.json)
  and
  [`/schemas/managed/usage_export_row.schema.json`](../schemas/managed/usage_export_row.schema.json);
  worked cases in
  [`/fixtures/managed/metering_cases/`](../fixtures/managed/metering_cases/).

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

- [`observability/observability_signal_contract.md`](./observability/observability_signal_contract.md)
  — shared signal-slice, source, freshness, partial-evidence,
  correlation, and export/share vocabulary for logs, metrics, traces,
  incident timelines, and post-incident evidence packets. Boundary
  schemas in
  [`/schemas/observability/`](../schemas/observability/);
  worked slices in
  [`/fixtures/observability/signal_slice_cases/`](../fixtures/observability/signal_slice_cases/).
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
- [`docs_integrity/citation_and_reference_contract.md`](./docs_integrity/citation_and_reference_contract.md)
  — citation-anchor and symbol-linked-reference contract shared by
  docs search, docs panes, guided tours, glossary cards, AI
  explanations, hosted-review evidence, repair packets, and
  portability/offboarding exports. Boundary schemas in
  [`/schemas/docs/citation_anchor.schema.json`](../schemas/docs/citation_anchor.schema.json)
  and
  [`/schemas/docs/symbol_linked_reference.schema.json`](../schemas/docs/symbol_linked_reference.schema.json);
  worked fixtures in
  [`/fixtures/docs/citation_cases/`](../fixtures/docs/citation_cases/).
- [`docs_integrity/assist_to_help_bridge_contract.md`](./docs_integrity/assist_to_help_bridge_contract.md)
  — assist-to-help bridge contract preserving symbol/file identity,
  source class, mapping quality, version/freshness/locale posture,
  citation retention, browser/provider handoff reason, and return
  context when hover, peek, inline assist, AI explanation, onboarding,
  glossary, docs, or support/export surfaces open help. Boundary
  schemas in
  [`/schemas/docs/assist_reference.schema.json`](../schemas/docs/assist_reference.schema.json)
  and
  [`/schemas/docs/assist_help_handoff.schema.json`](../schemas/docs/assist_help_handoff.schema.json);
  worked fixtures in
  [`/fixtures/docs/assist_reference_cases/`](../fixtures/docs/assist_reference_cases/).
- [`docs/reviewed_pack_and_late_copy_policy.md`](./docs/reviewed_pack_and_late_copy_policy.md)
  — reviewed-pack and controlled late-copy contract for release-bearing
  trust, legal, policy, recovery, support, and compatibility copy.
  Freezes one reviewed-pack version model, the binding-state labels
  (`reviewed_current`, `stale_reviewed_source`,
  `late_copy_override_active`, `late_copy_override_reversed`,
  `blocked_unreviewed`), the late-copy reason classes, required
  reviewer sets, and rollback/reversal notes that docs/help,
  migration, support-export, release-note, CLI/help, evaluation, and
  public-proof lanes reuse after string freeze. Boundary schema in
  [`/schemas/docs/late_copy_change_packet.schema.json`](../schemas/docs/late_copy_change_packet.schema.json);
  worked fixtures in
  [`/fixtures/docs/late_copy_examples/`](../fixtures/docs/late_copy_examples/).
- [`copy/translation_safe_content_ops_contract.md`](./copy/translation_safe_content_ops_contract.md)
  — translation-safe content-ops contract for placeholder semantics,
  translator-note preservation, glossary refs, screenshot/demo caption
  metadata, source-language fallback, pseudoloc/truncation gates, and
  late-copy downstream impact. Boundary schemas in
  [`/schemas/ux/message_placeholder.schema.json`](../schemas/ux/message_placeholder.schema.json)
  and
  [`/schemas/copy/late_copy_change.schema.json`](../schemas/copy/late_copy_change.schema.json);
  worked fixtures in
  [`/fixtures/copy/placeholder_and_late_copy_cases/`](../fixtures/copy/placeholder_and_late_copy_cases/).
- [`copy/ui_copy_contract.md`](./copy/ui_copy_contract.md)
  — unified UI copy contract for action labels, error messages, and AI
  copy guardrails. Machine-readable lint rules in
  [`/artifacts/copy/ui_copy_lint_rules.yaml`](../artifacts/copy/ui_copy_lint_rules.yaml);
  boundary schema in
  [`/schemas/copy/error_message.schema.json`](../schemas/copy/error_message.schema.json);
  worked fixtures in
  [`/fixtures/copy/ui_copy_cases/`](../fixtures/copy/ui_copy_cases/).
- [`copy/count_scope_freshness_grammar.md`](./copy/count_scope_freshness_grammar.md)
  — copy grammar and term contract for dense count, scope, freshness,
  and chronology labels. Freezes controlled meanings for visible,
  loaded, all matching, selected, hidden by policy, outside current
  workset, approx., exact, partial, cached, streaming, warming, stale,
  provider-limited, and unknown copy so search summaries, selection
  bars, batch actions, queue rows, dashboard cards, export headers, CLI
  summaries, accessibility labels, and support exports cannot overclaim
  results, selected membership, synced state, or event outcomes.
  Boundary schema in
  [`/schemas/copy/microcopy_term.schema.json`](../schemas/copy/microcopy_term.schema.json);
  machine-readable term set in
  [`/artifacts/copy/count_scope_term_set.yaml`](../artifacts/copy/count_scope_term_set.yaml);
  worked fixtures in
  [`/fixtures/copy/microcopy_cases/`](../fixtures/copy/microcopy_cases/).
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
- [`search/search_query_session_contract.md`](./search/search_query_session_contract.md)
  — canonical search query-session, result-identity, and
  explanation-capture contract shared by quick open, global search,
  docs search, AI retrieval, CLI search, saved-query reopen, and
  support export. Boundary schemas in
  [`/schemas/search/query_session.schema.json`](../schemas/search/query_session.schema.json),
  [`/schemas/search/search_result_identity.schema.json`](../schemas/search/search_result_identity.schema.json),
  and
  [`/schemas/search/search_explanation_capture.schema.json`](../schemas/search/search_explanation_capture.schema.json);
  worked cases in
  [`/fixtures/search/query_session_cases/`](../fixtures/search/query_session_cases/).
- [`ux/search_result_contract.md`](./ux/search_result_contract.md)
  — renderer-facing search result row, guarantee label, quick-action,
  replace-scope, saved-query, and deep-link route contract. Boundary
  schema in
  [`/schemas/search/search_result_row.schema.json`](../schemas/search/search_result_row.schema.json);
  worked fixtures in
  [`/fixtures/search/result_cases/`](../fixtures/search/result_cases/).
- [`ux/quick_open_contract.md`](./ux/quick_open_contract.md)
  — quick-open result-row, readiness-banner, same-surface
  explanation, and focus-return contract. Boundary schemas in
  [`/schemas/search/quick_open_result.schema.json`](../schemas/search/quick_open_result.schema.json)
  and
  [`/schemas/search/quick_open_explanation.schema.json`](../schemas/search/quick_open_explanation.schema.json);
  worked fixtures in
  [`/fixtures/search/quick_open_rows/`](../fixtures/search/quick_open_rows/).
- [`ux/breadcrumb_contract.md`](./ux/breadcrumb_contract.md)
  — renderer-facing breadcrumb segment, overflow, keyboard, action,
  stale-state, and cursor-driven symbol update contract layered over the
  durable navigation trail. Boundary schema in
  [`/schemas/navigation/breadcrumb_segment.schema.json`](../schemas/navigation/breadcrumb_segment.schema.json);
  worked fixtures in
  [`/fixtures/navigation/breadcrumb_examples/`](../fixtures/navigation/breadcrumb_examples/).
- [`navigation/semantic_navigation_and_rename_contract.md`](./navigation/semantic_navigation_and_rename_contract.md)
  — durable semantic-result identity and rename-preview contract for
  definition, declaration, type-definition, implementation, reference,
  hierarchy, call-site, alias, imported/generated reference, review,
  support-export, and AI-citation lanes. Boundary schemas in
  [`/schemas/navigation/semantic_result_ref.schema.json`](../schemas/navigation/semantic_result_ref.schema.json)
  and
  [`/schemas/navigation/rename_preview.schema.json`](../schemas/navigation/rename_preview.schema.json);
  worked fixtures in
  [`/fixtures/navigation/semantic_navigation_cases/`](../fixtures/navigation/semantic_navigation_cases/).
- [`architecture/language_protocol_router_adr.md`](./architecture/language_protocol_router_adr.md)
  — typed language protocol-router ADR seed for LSP, DAP, formatter,
  linter, test, build, framework, native analyzer, project graph,
  generated-source bridge, syntax, and assist providers. Freezes
  provider capability descriptors, resolution records, precedence,
  fallback, coexistence, health scoring, crash-loop quarantine,
  placement, and coordinate-translation requirements. Boundary schemas
  in
  [`/schemas/language/provider_capability.schema.json`](../schemas/language/provider_capability.schema.json)
  and
  [`/schemas/language/provider_resolution.schema.json`](../schemas/language/provider_resolution.schema.json);
  worked fixtures in
  [`/fixtures/language/router_cases/`](../fixtures/language/router_cases/).
- [`language/provider_graph_and_arbitration_contract.md`](./language/provider_graph_and_arbitration_contract.md)
  — shared language-provider attribution, capability-negotiation,
  arbitration, and result-provenance contract for definition,
  reference, hover, rename, completion, code-action, diagnostics,
  notebook context, and AI assistance. Boundary schemas in
  [`/schemas/language/provider_status_row.schema.json`](../schemas/language/provider_status_row.schema.json),
  [`/schemas/language/capability_negotiation_packet.schema.json`](../schemas/language/capability_negotiation_packet.schema.json),
  and
  [`/schemas/language/result_provenance.schema.json`](../schemas/language/result_provenance.schema.json);
  worked fixtures in
  [`/fixtures/language/provider_arbitration_cases/`](../fixtures/language/provider_arbitration_cases/).
- [`language/three_layer_model_contract.md`](./language/three_layer_model_contract.md)
  — the canonical three-layer language intelligence model (syntax/structure,
  compatibility/breadth, Aureline-owned semantic depth) and the cross-surface
  rule that every answer discloses its layer and downgrade reason. Companion
  matrix in
  [`/artifacts/language/layer_matrix.yaml`](../artifacts/language/layer_matrix.yaml);
  worked fixtures in
  [`/fixtures/language/layer_cases/`](../fixtures/language/layer_cases/).
- [`language/completion_and_inline_hint_contract.md`](./language/completion_and_inline_hint_contract.md)
  — shared completion-row, signature-help, snippet-session, code-lens,
  and inline-hint contract for the typing loop, including source
  labeling, insert/commit posture, side-effect cues, ranking
  attribution, active-parameter visibility, inline precedence, and
  density-aware suppression. Boundary schemas in
  [`/schemas/language/completion_row.schema.json`](../schemas/language/completion_row.schema.json),
  [`/schemas/language/signature_help_state.schema.json`](../schemas/language/signature_help_state.schema.json),
  and
  [`/schemas/language/inline_hint_state.schema.json`](../schemas/language/inline_hint_state.schema.json);
  worked fixtures in
  [`/fixtures/language/completion_hint_cases/`](../fixtures/language/completion_hint_cases/).
- [`language/diagnostics_and_code_action_contract.md`](./language/diagnostics_and_code_action_contract.md)
  — shared diagnostic source taxonomy, semantic-layer state, clustering,
  code-action summary, and suppression/baseline governance contract.
  Boundary schemas in
  [`/schemas/language/diagnostic_cluster.schema.json`](../schemas/language/diagnostic_cluster.schema.json),
  [`/schemas/language/code_action_summary.schema.json`](../schemas/language/code_action_summary.schema.json),
  and
  [`/schemas/language/suppression_review.schema.json`](../schemas/language/suppression_review.schema.json);
  worked fixtures in
  [`/fixtures/language/diagnostic_convergence_cases/`](../fixtures/language/diagnostic_convergence_cases/).
- [`language/diagnostic_freshness_and_delta_contract.md`](./language/diagnostic_freshness_and_delta_contract.md)
  — finding-level diagnostic remap, SARIF-like/provider import, and
  current/imported/baseline/suppression delta parity contract for
  editor, review, CLI/headless, release, and support surfaces. Boundary
  schemas in
  [`/schemas/language/diagnostic_remap_state.schema.json`](../schemas/language/diagnostic_remap_state.schema.json),
  [`/schemas/language/sarif_import_record.schema.json`](../schemas/language/sarif_import_record.schema.json),
  and
  [`/schemas/language/diagnostic_delta.schema.json`](../schemas/language/diagnostic_delta.schema.json);
  worked fixtures in
  [`/fixtures/language/diagnostic_delta_cases/`](../fixtures/language/diagnostic_delta_cases/).
- [`ux/editor_anatomy_contract.md`](./ux/editor_anatomy_contract.md)
  — editor anatomy, layer-classification, and stable-text-column
  contract for document header, context layer, gutter, text viewport,
  inline assist, transient knowledge, and status layer ownership.
  Freezes content / annotation / action-affordance classification,
  no-jitter source-column behavior, required placement for compare,
  restored, generated, read-only, live-preview, degraded, and large-file
  states, and large-file/degraded layer downgrade disclosure. Boundary
  schema in
  [`/schemas/ux/editor_layer.schema.json`](../schemas/ux/editor_layer.schema.json);
  worked fixtures in
  [`/fixtures/ux/editor_layer_cases/`](../fixtures/ux/editor_layer_cases/).
- [`ux/editor_document_state_contract.md`](./ux/editor_document_state_contract.md)
  — editor document-state badge contract for dirty, pinned, compare,
  recovered-snapshot, generated-output, imported-patch, read-only,
  mirrored, policy-locked, live-preview, conflict, and stale states.
  Freezes required placement across tabs, document headers,
  breadcrumbs/context, status surfaces, compare sheets, preview sheets,
  support bundles, accessibility labels, and docs screenshots; also
  freezes canonical recovery/source actions and identity-preservation
  rules across splits, restores, compare mode, and preview promotion.
  Boundary schema in
  [`/schemas/editor/document_state_badge.schema.json`](../schemas/editor/document_state_badge.schema.json);
  worked fixtures in
  [`/fixtures/editor/document_state_cases/`](../fixtures/editor/document_state_cases/).
- [`ux/file_state_badge_and_write_review_contract.md`](./ux/file_state_badge_and_write_review_contract.md)
  — cross-surface file-state badge, reason-strip, and write-review
  contract for read-only, generated, policy-locked, managed/mirrored,
  projection, and captured-snapshot states. Freezes source-of-truth
  relation, write authority, dirty state, freshness, next-safe action,
  write-review side effects, and checkpoint/rollback posture for
  editor, diff, preview, review, notebook, and evidence surfaces.
  Boundary schemas in
  [`/schemas/ux/file_state_badge_group.schema.json`](../schemas/ux/file_state_badge_group.schema.json)
  and
  [`/schemas/ux/write_review_sheet.schema.json`](../schemas/ux/write_review_sheet.schema.json);
  worked fixtures in
  [`/fixtures/ux/file_state_surface_cases/`](../fixtures/ux/file_state_surface_cases/).
- [`ux/editor_external_change_contract.md`](./ux/editor_external_change_contract.md)
  — editor external-change contract for recovery-safe handling when a
  file changes outside Aureline, watcher/root state is uncertain, an
  alias path changes, or save would overwrite newer external content.
  Freezes compare-or-reload state classes, stale-buffer disclosure,
  save-blocked surfacing, checkpoint/undo expectations, special
  removable-volume/case-only-rename/symlink/remote-root rules, and the
  review-choice matrix for compare, overwrite, merge, cancel, reload,
  and retry. Boundary schema in
  [`/schemas/editor/external_change_event.schema.json`](../schemas/editor/external_change_event.schema.json);
  worked fixtures in
  [`/fixtures/editor/external_change_cases/`](../fixtures/editor/external_change_cases/).
- [`ux/editor_gutter_contract.md`](./ux/editor_gutter_contract.md)
  — editor gutter lane, precedence, hit-target, accessibility, and
  no-jitter contract for line numbers, execution/debug markers,
  diagnostics, changes/review, fold controls, and supplemental cues.
  Freezes signal admission/rejection, breakpoint-versus-diagnostic,
  fold-versus-execution, high-severity precedence, keyboard-equivalent
  rules, and narrow-width fallback. Boundary schema in
  [`/schemas/ux/editor_gutter_lane.schema.json`](../schemas/ux/editor_gutter_lane.schema.json);
  worked fixtures in
  [`/fixtures/ux/gutter_cases/`](../fixtures/ux/gutter_cases/).
- [`ux/editor_inline_assist_contract.md`](./ux/editor_inline_assist_contract.md)
  — editor inline assist contract for diagnostics, debug/current-frame
  state, review/diff state, test/coverage cues, code lenses, inlay
  hints, ghost text, inline values, inline quick actions, precedence,
  density reduction, stale/approximate labels, and accessibility
  parity. Boundary schema in
  [`/schemas/editor/inline_assist.schema.json`](../schemas/editor/inline_assist.schema.json);
  worked fixtures in
  [`/fixtures/editor/inline_assist_cases/`](../fixtures/editor/inline_assist_cases/).
- [`ux/editor_selection_contract.md`](./ux/editor_selection_contract.md)
  — source-editor selection, caret, multi-cursor, column-selection,
  line-selection, structural/semantic selection, IME composition, and
  scope-status contract. Freezes editor-specific labels for cursor
  counts, visible versus all-matching ranges, selection-derived writes,
  explicit scope widening, undo/preview/snippet/modal persistence, and
  read-only/generated compare behavior. Boundary schema in
  [`/schemas/editor/selection_state.schema.json`](../schemas/editor/selection_state.schema.json);
  worked fixtures in
  [`/fixtures/editor/selection_cases/`](../fixtures/editor/selection_cases/).
- [`verification/source_fidelity_and_undo_packet.md`](./verification/source_fidelity_and_undo_packet.md)
  — seed verification packet for save/source-fidelity fields,
  whole-file-rewrite disclosure, and undo-honesty copy. Pairs with the
  machine-readable corpus in
  [`/fixtures/io/source_fidelity_corpus_manifest.yaml`](../fixtures/io/source_fidelity_corpus_manifest.yaml),
  the rewrite-class vocabulary in
  [`/artifacts/io/save_rewrite_classes.yaml`](../artifacts/io/save_rewrite_classes.yaml),
  and the worked records in
  [`/artifacts/io/undo_recovery_examples/`](../artifacts/io/undo_recovery_examples/).
- [`verification/reactive_state_packet.md`](./verification/reactive_state_packet.md)
  — seed verification packet for reactive-state parity,
  stale/partial/replayed/failed-refresh labeling, cross-surface
  query-family identifiers, and invalidation-order audits. Pairs with
  the parity corpus in
  [`/fixtures/state/snapshot_delta_parity_manifest.yaml`](../fixtures/state/snapshot_delta_parity_manifest.yaml),
  the query-family examples in
  [`/artifacts/state/query_family_examples/`](../artifacts/state/query_family_examples/),
  and the condensed order audits in
  [`/artifacts/state/invalidation_order_trace_examples/`](../artifacts/state/invalidation_order_trace_examples/).
- [`verification/focus_and_batch_scope_packet.md`](./verification/focus_and_batch_scope_packet.md)
  — seed verification packet for dense-collection focus return,
  selected/visible/loaded/matching truth, hidden-selected disclosure,
  blocked-versus-skipped separation, and range-selection accessibility.
  Pairs with the machine-readable corpus in
  [`/fixtures/ux/selection_and_virtualization_manifest.yaml`](../fixtures/ux/selection_and_virtualization_manifest.yaml),
  the focus-return examples in
  [`/artifacts/ux/focus_return_examples/`](../artifacts/ux/focus_return_examples/),
  and the assistive-tech cases in
  [`/artifacts/accessibility/range_selection_at_cases/`](../artifacts/accessibility/range_selection_at_cases/).
- [`verification/policy_simulation_packet.md`](./verification/policy_simulation_packet.md)
  — seed verification packet for policy-simulation diffs,
  remembered-decision narrowing, waiver-expiry dashboard joins, and
  timezone-aware chronology-bar export truth. Pairs with the machine-
  readable diff corpus in
  [`/fixtures/policy/simulation_diff_manifest.yaml`](../fixtures/policy/simulation_diff_manifest.yaml),
  the chronology-bar cases in
  [`/fixtures/policy/chronology_bar_cases/`](../fixtures/policy/chronology_bar_cases/),
  and the dashboard field contract in
  [`/artifacts/policy/waiver_expiry_dashboard_contract.yaml`](../artifacts/policy/waiver_expiry_dashboard_contract.yaml).
- [`architecture/generated_artifact_safe_edit_policy.md`](./architecture/generated_artifact_safe_edit_policy.md)
  — shared safe-edit posture for generated, mirrored, imported, and
  preview artifacts. Freezes the compact cross-surface posture record
  naming artifact origin, provenance/drift state, default edit posture,
  rebuild intent, override review/provenance requirements, and
  structured-viewer fallback. Boundary schema in
  [`/schemas/generated/artifact_edit_posture.schema.json`](../schemas/generated/artifact_edit_posture.schema.json);
  reviewer corpus in
  [`/fixtures/generated/drift_regeneration_manifest.yaml`](../fixtures/generated/drift_regeneration_manifest.yaml);
  worked posture examples in
  [`/artifacts/generated/viewer_fallback_examples/`](../artifacts/generated/viewer_fallback_examples/).
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
- [`ux/overlay_layer_contract.md`](./ux/overlay_layer_contract.md)
  — overlay-layer, portal-boundary, scrim, focus-trap,
  dismissal/back-stack, nesting, and promotion contract shared by
  tooltips, hovercards, popovers, peeks, menus, dialogs, sheets,
  banners, critical prompts, and promoted panels. Boundary schema in
  [`/schemas/ux/overlay_stack.schema.json`](../schemas/ux/overlay_stack.schema.json);
  worked fixtures in
  [`/fixtures/ux/overlay_cases/`](../fixtures/ux/overlay_cases/).
- [`ux/shell_close_reopen_contract.md`](./ux/shell_close_reopen_contract.md)
  — shell-slot close/reopen, remembered-last-surface, placeholder, and
  focus-return contract for rail sections, sidebars, workspace
  surfaces, inspectors, bottom-panel tabs, collapsed panes, status
  routes, and transient overlays. Boundary schema in
  [`/schemas/ux/shell_slot_memory.schema.json`](../schemas/ux/shell_slot_memory.schema.json);
  worked fixtures in
  [`/fixtures/ux/shell_slot_cases/`](../fixtures/ux/shell_slot_cases/).
- [`ux/splitter_contract.md`](./ux/splitter_contract.md)
  — splitter and resizable-pane contract covering visible line versus
  hit target, hover/focus reinforcement, keyboard fine and coarse
  resize, reset/equalize behavior, screen-reader controlled-region
  naming, recoverable collapse, no-silent-collapse barriers, and
  proportional or preset-based persistence. Boundary schema in
  [`/schemas/ux/splitter_state.schema.json`](../schemas/ux/splitter_state.schema.json);
  worked fixtures in
  [`/fixtures/ux/splitter_cases/`](../fixtures/ux/splitter_cases/).
- [`ux/status_bar_contract.md`](./ux/status_bar_contract.md)
  — status bar priority, stable-slot, compact overflow, search/menu
  parity, extension contribution budget, and anti-jitter contract for
  recovery-critical state, active context truth, ongoing work, and
  ambient metadata. Boundary schema in
  [`/schemas/ux/status_item.schema.json`](../schemas/ux/status_item.schema.json);
  worked fixtures in
  [`/fixtures/ux/status_items/`](../fixtures/ux/status_items/).
- [`ux/durable_work_contract.md`](./ux/durable_work_contract.md) -
  durable-work row contract for long-running, queued, approval-
  blocked, attention-required, completed, partially completed, quiet-
  hours held, and policy-suppressed work. Freezes state classes,
  progress forms, activity-center partitions, notification linkbacks,
  and support-export fields. Boundary schema in
  [`/schemas/ux/job_row.schema.json`](../schemas/ux/job_row.schema.json);
  worked fixtures in
  [`/fixtures/ux/job_rows/`](../fixtures/ux/job_rows/).
- [`ux/status_strip_family_contract.md`](./ux/status_strip_family_contract.md)
  — shared top-of-surface status-strip and readiness-banner family for
  workspace, environment, provider, graph, and framework surfaces.
  Freezes one seven-value readiness vocabulary, shared scope/count/action
  anatomy, hidden-scope and partial/stale top-level disclosure,
  surface-specific variant blocks, and export/screenshot-safe rules for
  live, cached, provider-backed, partial, stale, degraded, blocked, and
  ready states. Boundary schema in
  [`/schemas/ux/status_strip.schema.json`](../schemas/ux/status_strip.schema.json);
  worked fixtures in
  [`/fixtures/ux/status_strips/`](../fixtures/ux/status_strips/).
- [`ux/live_update_review_contract.md`](./ux/live_update_review_contract.md)
  — shared live-update review contract covering pause/freeze,
  buffered-vs-stale honesty, anchor stability, batch-membership drift,
  provider-limited visibility, snapshot review, and copy/export scope
  for dense tables, result grids, logs, and streaming timelines.
  Boundary schema in
  [`/schemas/ux/live_set_state.schema.json`](../schemas/ux/live_set_state.schema.json);
  worked fixtures in
  [`/fixtures/ux/live_review_examples/`](../fixtures/ux/live_review_examples/).
- [`ux/view_freshness_contract.md`](./ux/view_freshness_contract.md)
  — cross-surface freshness, materialized-view disclosure, and
  captured-versus-live scope contract for search, docs, graph, logs,
  review packs, notebooks, dashboards, support packets, and other
  materialized views. Boundary schema in
  [`/schemas/ux/view_freshness.schema.json`](../schemas/ux/view_freshness.schema.json);
  worked fixtures in
  [`/fixtures/ux/view_freshness_cases/`](../fixtures/ux/view_freshness_cases/).
- [`ux/empty_loading_placeholder_contract.md`](./ux/empty_loading_placeholder_contract.md)
  — cross-surface empty, loading, skeleton, progressive hydration,
  missing-provider, partial-data, and stale-cached placeholder-honesty
  contract. Freezes shared readiness language and the rule that
  placeholder surfaces cannot present ready/green state while the
  underlying scope is probing, partial, blocked, degraded, stale, or
  provider-limited. Boundary schema in
  [`/schemas/ux/placeholder_state.schema.json`](../schemas/ux/placeholder_state.schema.json);
  worked fixtures in
  [`/fixtures/ux/placeholder_cases/`](../fixtures/ux/placeholder_cases/).
- [`ux/tree_row_contract.md`](./ux/tree_row_contract.md)
  — shared structural tree row contract for file trees, outlines,
  component/runtime trees, route and dependency trees, write-scope
  preview trees, and support/export projections. Freezes row anatomy,
  partial-readiness placeholders, hidden-scope disclosures, selection
  sync, read-only/generated/locked states, search-match highlights, and
  identity recovery for moved, missing, imported, cached, generated, and
  unsupported nodes. Boundary schema in
  [`/schemas/ux/tree_row.schema.json`](../schemas/ux/tree_row.schema.json);
  worked fixtures in
  [`/fixtures/ux/tree_rows/`](../fixtures/ux/tree_rows/).
- [`ux/tree_view_contract.md`](./ux/tree_view_contract.md)
  — shared hierarchy interaction contract for file, outline, schema,
  package, component/runtime, route, dependency, write-scope preview,
  and support/export trees. Freezes indentation, disclosure, lazy
  hydration, keyboard navigation, active/current/selected/open state,
  virtualization, batch selection, drag or move posture, inline versus
  deferred actions, and provider fallback. Boundary schema in
  [`/schemas/ux/tree_row.schema.json`](../schemas/ux/tree_row.schema.json);
  worked fixtures in
  [`/fixtures/ux/tree_view_cases/`](../fixtures/ux/tree_view_cases/).
- [`ux/forms_validation_contract.md`](./ux/forms_validation_contract.md)
  — shared form-validation, async-probe, and staged-review contract
  for settings, connections, packages, policy edits, repairs,
  transport forms, request/runtime forms, and secret-bearing or
  external-target validation flows. Boundary schemas in
  [`/schemas/ux/form_probe_state.schema.json`](../schemas/ux/form_probe_state.schema.json)
  and
  [`/schemas/ux/staged_review_state.schema.json`](../schemas/ux/staged_review_state.schema.json);
  worked fixtures in
  [`/fixtures/ux/form_validation_cases/`](../fixtures/ux/form_validation_cases/).
- [`ux/degraded_mode_pattern.md`](./ux/degraded_mode_pattern.md)
  — degraded-mode template and lifecycle-status card family for
  workspaces, extensions, remote sessions, collaboration sessions, AI
  actions, and update or rollback flows. Freezes controlled labels,
  preserved-vs-reduced capability slots, last-failure visibility,
  keyboard-reachable inspect paths, and safe recovery actions that
  preserve current work. Boundary schema in
  [`/schemas/ux/lifecycle_status_card.schema.json`](../schemas/ux/lifecycle_status_card.schema.json);
  worked fixtures in
  [`/fixtures/ux/degraded_examples/`](../fixtures/ux/degraded_examples/).
- [`ux/component_contract_template.md`](./ux/component_contract_template.md)
  — reusable component-contract packet for anatomy, explicit
  state-machine rows, content rules, keyboard behavior, accessibility,
  token and density bindings, theme/icon/motion hooks, localization
  behavior, extension guidance, substitution visibility, and typed
  evidence hooks. Boundary schema in
  [`/schemas/design/component_contract.schema.json`](../schemas/design/component_contract.schema.json);
  worked fixtures in
  [`/fixtures/design/component_contract_examples/`](../fixtures/design/component_contract_examples/).
- [`ux/feature_readiness_checklist.md`](./ux/feature_readiness_checklist.md)
  and
  [`ux/design_release_evidence_pack_template.md`](./ux/design_release_evidence_pack_template.md)
  — launch-critical feature readiness gate and reusable design release
  evidence-pack template covering state sets, keyboard and command maps,
  accessibility, theme captures, performance/efficiency, policy/trust,
  telemetry, rollout, migration, extension, compatibility, public-proof,
  and waiver refs. Machine-readable gate manifest in
  [`/artifacts/ux/review_gate_manifest.yaml`](../artifacts/ux/review_gate_manifest.yaml).
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
- [`security/threat_model_and_audit_stream_contract.md`](./security/threat_model_and_audit_stream_contract.md)
  — shared threat-class, audit-stream, evidence-window, redaction,
  export, and omission-disposition contract for advisories, incidents,
  approval tickets, support exports, collaboration elevated-control
  grants, remote join/leave events, and managed tenant/key reviews.
  Machine-readable companions live in
  [`/artifacts/security/threat_classes.yaml`](../artifacts/security/threat_classes.yaml),
  [`/schemas/security/audit_stream_record.schema.json`](../schemas/security/audit_stream_record.schema.json),
  and
  [`/schemas/security/evidence_window.schema.json`](../schemas/security/evidence_window.schema.json);
  worked fixtures live in
  [`/fixtures/security/audit_stream_cases/`](../fixtures/security/audit_stream_cases/).
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
- [`ai/ai_copy_guardrails_contract.md`](./ai/ai_copy_guardrails_contract.md)
  — AI copy contract for evidence-first confidence wording, preferred
  and forbidden AI terms, uncertainty / partial-context /
  omitted-context / stale-doc / replay / provider-route disclosure, and
  the separation between `Explain`, `Open source`, `Prepare preview`,
  `Open diff`, `Start sandbox run`, and direct mutation controls.
  Machine-readable term registries live in
  [`/artifacts/ai/approved_ai_terms.yaml`](../artifacts/ai/approved_ai_terms.yaml)
  and
  [`/artifacts/ai/forbidden_ai_terms.yaml`](../artifacts/ai/forbidden_ai_terms.yaml);
  worked fixtures live in
  [`/fixtures/ai/copy_guardrail_cases/`](../fixtures/ai/copy_guardrail_cases/).
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
- [`profile/profile_sync_and_conflict_contract.md`](./profile/profile_sync_and_conflict_contract.md)
  — profile-library, optional-sync metadata, machine-binding addendum,
  and conflict-resolution contract. Freezes the six profile scope
  classes (profile defaults, user-global preferences, machine-specific
  state, workspace settings, administrative defaults / policy, and
  ephemeral session state), the profile-library entry row, profile
  conflict-journal row, non-widening import / sync rules, merge-preview
  and rollback expectations, no-service portability fallback, and the
  minimum posture required before encrypted or customer-managed sync
  storage can be claimed. Boundary schemas in
  [`/schemas/profile/profile_library_entry.schema.json`](../schemas/profile/profile_library_entry.schema.json)
  and
  [`/schemas/profile/sync_conflict_record.schema.json`](../schemas/profile/sync_conflict_record.schema.json);
  worked fixtures in
  [`/fixtures/profile/scope_class_cases/`](../fixtures/profile/scope_class_cases/).
- [`ux/persistence_inspector_contract.md`](./ux/persistence_inspector_contract.md)
  — concrete remembered-state inspector, portable-state export sheet,
  and restore-provenance card contract for persistence review. Freezes
  row fields for artifact class, last-write time, schema version,
  restore fidelity, portability label, redaction, size, checksum /
  signature state, local-only exclusions, inspect / export / compare /
  clear actions, and restore cards that explain live, placeholder,
  context-only, blocked, and intentionally excluded state. Boundary
  schema in
  [`/schemas/state/portable_state_package.schema.json`](../schemas/state/portable_state_package.schema.json);
  worked restore-card fixtures in
  [`/fixtures/state/restore_provenance_cards/`](../fixtures/state/restore_provenance_cards/).
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
- [`ux/workspace_entry_route_matrix.md`](./ux/workspace_entry_route_matrix.md)
  — workspace-entry route matrix and safe-open / restricted-open /
  restore transition contract. Freezes route-level semantics for
  Open folder, Open workspace, Clone repository, Import, Resume
  snapshot, Restore last session, Deep link, Open in safe mode, and
  Continue in restricted mode; requires previews for target identity,
  trust/policy boundary changes, destinations, side effects, imported
  artifact classes, restore classes, missing prerequisites, and
  fallback actions before commit; and defines parity across Start
  Center, main menu, command palette, deep-link resolver, and
  workspace switcher. Boundary schema in
  [`/schemas/workspace/entry_route.schema.json`](../schemas/workspace/entry_route.schema.json);
  worked fixtures in
  [`/fixtures/workspace/entry_route_cases/`](../fixtures/workspace/entry_route_cases/).
- [`workspace/source_acquisition_and_bootstrap_seed.md`](./workspace/source_acquisition_and_bootstrap_seed.md)
  and
  [`workspace/bootstrap_packet_contract.md`](./workspace/bootstrap_packet_contract.md)
  — source-locator, checkout-plan, bootstrap-queue, and bootstrap
  packet contracts for open / clone / import / resume provenance,
  trust stage, mirror/public route, credential-handle refs,
  resumability, partial failures, post-open prerequisites, export
  rules, and typed reason codes. Boundary schemas in
  [`/schemas/workspace/source_locator.schema.json`](../schemas/workspace/source_locator.schema.json),
  [`/schemas/workspace/checkout_plan.schema.json`](../schemas/workspace/checkout_plan.schema.json),
  [`/schemas/workspace/bootstrap_queue_item.schema.json`](../schemas/workspace/bootstrap_queue_item.schema.json),
  and
  [`/schemas/workspace/bootstrap_packet.schema.json`](../schemas/workspace/bootstrap_packet.schema.json);
  reason codes in
  [`/artifacts/workspace/bootstrap_reason_codes.yaml`](../artifacts/workspace/bootstrap_reason_codes.yaml);
  worked fixtures in
  [`/fixtures/workspace/bootstrap_cases/`](../fixtures/workspace/bootstrap_cases/).
- [`templates/template_registry_and_scaffold_contract.md`](./templates/template_registry_and_scaffold_contract.md)
  — template-registry entry, scaffold-hook policy, and generated-project
  update semantics contract. Freezes registry identity, signing-root /
  trust-source, mirror/origin class, certification/support class,
  compatible runtime/schema ranges, health cadence, hook preview /
  network / credential / lineage policy, no-hidden-hook rules, and
  no-silent-overwrite reapply/update semantics. Boundary schemas in
  [`/schemas/templates/template_registry_entry.schema.json`](../schemas/templates/template_registry_entry.schema.json),
  [`/schemas/templates/scaffold_hook_policy.schema.json`](../schemas/templates/scaffold_hook_policy.schema.json),
  and
  [`/schemas/templates/generated_project_update_semantics.schema.json`](../schemas/templates/generated_project_update_semantics.schema.json);
  worked fixtures in
  [`/fixtures/templates/template_registry_cases/`](../fixtures/templates/template_registry_cases/).
- [`ux/archetype_detection_contract.md`](./ux/archetype_detection_contract.md)
  — post-entry workspace archetype detection, readiness-preflight,
  admission-checkpoint, first-useful-work routing, setup-later, and
  remembered-routing contract. Freezes the six detection outcomes
  (`certified_archetype_match`, `probable_archetype`,
  `mixed_or_ambiguous_workspace`, `unknown_or_generic_workspace`,
  `restricted_or_policy_blocked`, `missing_prerequisite`), material
  source-labeled detection signals, the three readiness buckets
  (`blocking_now`, `recommended_soon`, `optional_later`), and route
  expectations for single-file open, folder/repo open, clone,
  review/incident deep link, restore last session, and imported
  state/handoff packet entry. Boundary schema in
  [`/schemas/workspace/archetype_detection.schema.json`](../schemas/workspace/archetype_detection.schema.json);
  worked fixtures in
  [`/fixtures/workspace/entry_routes/`](../fixtures/workspace/entry_routes/).
- [`ux/recent_work_and_restore_card_contract.md`](./ux/recent_work_and_restore_card_contract.md)
  — concrete recent-work row, restore-card summary, and workspace-
  switcher row anatomy for startup and switching surfaces. Freezes
  primary labels, location / target subtitles, root kind, last-opened
  or last-validated times, trust state, restore availability,
  unavailable-target states, recovery actions
  (`open_read_only_cached_view`, `retry_later`, locate, reconnect,
  reauthorize, remove), privacy-reduction controls, write-safety
  badges, restore-card counts, dirty-buffer and remote-session
  summaries, and cross-window switching consequences. Boundary
  schema in
  [`/schemas/ux/recent_work_row.schema.json`](../schemas/ux/recent_work_row.schema.json);
  worked fixtures in
  [`/fixtures/ux/recent_work_rows/`](../fixtures/ux/recent_work_rows/).
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
- [`migration/compatibility_scorecard_contract.md`](./migration/compatibility_scorecard_contract.md)
  — imported-extension, imported-workflow, and workflow-bundle
  compatibility scorecard contract. Freezes supported, partial,
  community-path, blocked, deprecated, and replaced status semantics;
  caveat, owner, docs/help, compatibility-row, claim-manifest, support,
  evidence, and freshness fields; and downgrade behavior for stale or
  missing evidence. Boundary schema in
  [`/schemas/migration/compatibility_scorecard.schema.json`](../schemas/migration/compatibility_scorecard.schema.json);
  seed rows in
  [`/artifacts/migration/top_imported_workflow_rows.yaml`](../artifacts/migration/top_imported_workflow_rows.yaml);
  worked fixtures in
  [`/fixtures/migration/compatibility_scorecards/`](../fixtures/migration/compatibility_scorecards/).
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
- [`runtime/context_cache_and_terminal_restore_contract.md`](./runtime/context_cache_and_terminal_restore_contract.md)
  — execution-context cache and terminal-restore metadata contract.
  Freezes the derived cache entry, cache compare/reset preview,
  terminal restore metadata, restore compare/reset preview, and
  restore-no-rerun invariants that let Project Doctor and support
  flows diagnose wrong interpreter, wrong shell, wrong target, stale
  environment, and blocked restore cases without deleting broad state
  roots. Boundary schemas in
  [`/schemas/runtime/execution_context_cache_entry.schema.json`](../schemas/runtime/execution_context_cache_entry.schema.json)
  and
  [`/schemas/runtime/terminal_restore_metadata.schema.json`](../schemas/runtime/terminal_restore_metadata.schema.json);
  worked fixtures in
  [`/fixtures/runtime/context_cache_cases/`](../fixtures/runtime/context_cache_cases/).
- [`runtime/container_engine_and_preflight_contract.md`](./runtime/container_engine_and_preflight_contract.md)
  — container engine, devcontainer preflight, port-lease, and
  log-channel contract. Freezes engine support posture and capability
  flags for Docker, Podman, remote daemons, managed executors, custom
  backends, and unsupported backends; preflight findings for
  reachability, parse state, unsupported directives, mount/path risk,
  port collisions, mirror/credential prerequisites, and inspect-only
  fallback; and service-attributed port leases with route, restart,
  collision, viewer, evidence, and log-channel refs. Boundary schemas in
  [`/schemas/runtime/container_preflight_result.schema.json`](../schemas/runtime/container_preflight_result.schema.json)
  and
  [`/schemas/runtime/port_lease.schema.json`](../schemas/runtime/port_lease.schema.json);
  worked fixtures in
  [`/fixtures/runtime/container_preflight_cases/`](../fixtures/runtime/container_preflight_cases/).
- [`remote/attach_tunnel_port_forward_contract.md`](./remote/attach_tunnel_port_forward_contract.md)
  — remote attach, reconnect, tunnel, port-forward, service-discovery,
  and browser-preview route-truth contract. Freezes attach-session and
  forwarded-endpoint records with target identity, route origin,
  auth source, secret-exposure review, authority ticket, collision
  handling, stale-target labels, local-only versus shareable link
  disclosure, browser-handoff disclosure, lifecycle, recovery, and
  teardown semantics. Boundary schemas in
  [`/schemas/remote/attach_session.schema.json`](../schemas/remote/attach_session.schema.json)
  and
  [`/schemas/remote/forwarded_endpoint.schema.json`](../schemas/remote/forwarded_endpoint.schema.json);
  worked fixtures in
  [`/fixtures/remote/attach_cases/`](../fixtures/remote/attach_cases/).
- [`web/scoped_browser_surface_matrix.md`](./web/scoped_browser_surface_matrix.md)
  — scoped browser and embedded web surface capability matrix, local-core
  fallback postures, and disclosure rules shared by docs panes, auth
  handoff, preview/share, incident review, admin portals, companion
  notifications, and extension-hosted web surfaces. Machine-readable
  matrix in
  [`/artifacts/web/scoped_browser_capabilities.yaml`](../artifacts/web/scoped_browser_capabilities.yaml);
  worked examples in
  [`/fixtures/web/browser_surface_cases/`](../fixtures/web/browser_surface_cases/).
- [`runtime/resource_governor_contract.md`](./runtime/resource_governor_contract.md)
  — shared runtime resource-governor contract covering protected
  work classes, threshold families, queue and shed order,
  admission-control rules, and visible health-state semantics for
  `ready`, `warming`, `partial`, `degraded`, `offline`,
  `unsupported`, and `overloaded`. Machine-readable thresholds,
  policy fixtures, and overload scenarios live in
  [`/artifacts/runtime/resource_governor_thresholds.yaml`](../artifacts/runtime/resource_governor_thresholds.yaml).
- [`runtime/shared_state_machine_rules.md`](./runtime/shared_state_machine_rules.md)
  — shared cross-object state-class catalog and transition legend used
  to project workspace, extension, remote, collaboration, AI action,
  update/rollback, and long-running work state into one reusable set.
  Machine-readable catalog in
  [`/artifacts/runtime/state_catalog.yaml`](../artifacts/runtime/state_catalog.yaml);
  boundary schema in
  [`/schemas/runtime/state_transition_rule.schema.json`](../schemas/runtime/state_transition_rule.schema.json);
  worked fixtures in
  [`/fixtures/runtime/state_machine_examples/`](../fixtures/runtime/state_machine_examples/).
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
- [`performance/profiling_trace_replay_contract.md`](./performance/profiling_trace_replay_contract.md)
  — shared contract for governed CPU profiles, memory samples, render
  timelines, trace span sets, I/O captures, replay captures, regression
  baselines, comparison records, raw/summary export posture, retention,
  redaction, and artifact mismatch. Boundary schemas live in
  [`/schemas/performance/`](../schemas/performance/); class registry in
  [`/artifacts/performance/capture_classes.yaml`](../artifacts/performance/capture_classes.yaml);
  worked cases in
  [`/fixtures/performance/capture_cases/`](../fixtures/performance/capture_cases/).
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
- [`commands/palette_row_and_modifier_contract.md`](./commands/palette_row_and_modifier_contract.md)
  — combined command-palette row, modifier-action, and automation-cue
  contract for desktop palette rows, docs examples, CLI discovery,
  automation explainers, and support captures. Freezes required row
  elements (primary label, secondary scope detail, origin badge,
  winning shortcut hint, reason chip, automation labels, lifecycle cue,
  target/authority hint), modifier/footer actions, no-bypass guards,
  disabled/automation/command-ID cross-links, and hidden/deprecated/
  provider-backed/UI-only degraded states. Boundary schema in
  [`/schemas/commands/palette_row.schema.json`](../schemas/commands/palette_row.schema.json);
  worked fixtures in
  [`/fixtures/commands/palette_row_cases/`](../fixtures/commands/palette_row_cases/).
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
- [`commands/sequence_and_modal_discoverability_contract.md`](./commands/sequence_and_modal_discoverability_contract.md)
  — modal and sequence discoverability contract for current-mode,
  pending-operator, count, macro-recording, register-boundary, and
  leader-sequence cues; partial / ambiguous sequence help; shortcut
  teaching; imported-keymap conflict review; and parity between
  palette, leader overlays, modal sequences, colon-style command entry,
  docs, settings, and automation. Boundary schema in
  [`/schemas/commands/leader_overlay.schema.json`](../schemas/commands/leader_overlay.schema.json);
  worked examples in
  [`/fixtures/commands/sequence_help_examples/`](../fixtures/commands/sequence_help_examples/).
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
- [`design/semantic_token_domains_and_palette_contract.md`](./design/semantic_token_domains_and_palette_contract.md)
  — semantic token-domain ledger and palette contract that makes status,
  severity, syntax highlighting, diff roles, chart series roles, and
  trust/freshness/lifecycle cues mechanically attributable to one meaning
  owner. Machine-readable ledger in
  [`/artifacts/design/semantic_token_domains.yaml`](../artifacts/design/semantic_token_domains.yaml);
  boundary schema in
  [`/schemas/design/palette_mapping_row.schema.json`](../schemas/design/palette_mapping_row.schema.json);
  worked examples in
  [`/fixtures/design/palette_examples/`](../fixtures/design/palette_examples/).
- [`design/typography_text_contract.md`](./design/typography_text_contract.md)
  — typography roles, scale, font fallback posture, and overflow/copy-
  honesty contract for shell, docs/help, editor-adjacent UI, tables,
  badges, terminal metadata, and teaching surfaces. Canonical scale
  artifact in
  [`/artifacts/design/typography_scale.yaml`](../artifacts/design/typography_scale.yaml);
  boundary schema in
  [`/schemas/design/text_role.schema.json`](../schemas/design/text_role.schema.json);
  worked render cases in
  [`/fixtures/design/text_render_cases/`](../fixtures/design/text_render_cases/).
- [`ux/appearance_import_and_checkpoint_contract.md`](./ux/appearance_import_and_checkpoint_contract.md)
  — appearance-session, import-report, token-overlay, checkpoint,
  rollback, and extension/embedded-surface inheritance contract for
  theme changes. Boundary schemas in
  [`/schemas/ux/appearance_checkpoint.schema.json`](../schemas/ux/appearance_checkpoint.schema.json)
  and
  [`/schemas/ux/theme_import_report.schema.json`](../schemas/ux/theme_import_report.schema.json);
  worked fixtures in
  [`/fixtures/ux/appearance_cases/`](../fixtures/ux/appearance_cases/).

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
