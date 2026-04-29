# Launch-Critical Surface Traceability

This note is the reviewer-facing companion to
`artifacts/ux/surface_traceability_matrix.yaml`. It summarizes which
canonical ids and proof artifacts currently back each launch-critical
surface, how the surface degrades, and where the current seed still
depends on provisional or missing artifacts.

Design-complete gating for these rows lives in
[`artifacts/ux/review_gate_manifest.yaml`](../../artifacts/ux/review_gate_manifest.yaml).
Launch-critical rows remain blocked until the feature readiness
checklist is answered and a reviewable design release evidence pack
exists for the declared scope.

Cross-surface protected journeys (Project Doctor, refactor preview,
optional-cloud outage, generated-artifact drift, exposed-service
review, shared-session downgrade, mutation journal, deferred intent,
workspace-state restore provenance, support intake) are frozen in
[`docs/ux/pattern_inventory.md`](./pattern_inventory.md). Each
launch-critical surface here MUST also satisfy the obligations on its
row in
[`artifacts/ux/pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml);
surface-local exceptions require a visible freeze exception or waiver
ref.

Coverage markers:

- `canonical` means the surface already has a dedicated artifact, id, or packet family checked in.
- `provisional` means the artifact exists, but the proof is seeded, stitched, reserved, or broader than the exact surface.
- `missing` means the dependency is not seeded yet and review should treat the gap explicitly.

## Command palette

- Requirement ids: `FR-SHELL-001`, `A11Y-CORE-002`, `TOOL-CTX-002`, and `PERF-SHELL-001` (`provisional` for performance evidence).
- Canonical artifacts: `cmd:command_palette.open`, `discover:command_palette.open:01`, `command_palette.result_row`, `shell_conformance.quick_open_palette`, `tree_coverage.quick_open_palette`, `path.command_palette.open`.
- Degraded and recovery coverage: result rows stay explicit in `stale`, `restricted`, and `policy_blocked`; recovery routes are `cmd:search.refresh_scope`, `cmd:workspace.review_trust`, and `cmd:policy.view_effective_rule`.
- Evidence and gaps: component-contract fixture, command-registry row, accessibility task corpus, and perf evidence row exist; the perf family is still reserved/stitched and the three recovery commands are not yet seeded in the registry.

## Quick open

- Requirement ids: `FR-SHELL-001`, `FR-SEARCH-001`, `FR-SEARCH-002`, `PERF-SEARCH-003`, `PERF-INDEX-004`, `A11Y-CORE-002`, `ARCH-UX-005`.
- Canonical artifacts: search-result truth packet family, search-readiness vocabulary, `shell_conformance.quick_open_palette`, `tree_coverage.quick_open_palette`, and the shared command-discovery accessibility task.
- Degraded and recovery coverage: readiness and truth narrow through `partial_index`, `stale_index`, `semantic_unavailable_lexical_only`, and hidden-scope disclosure rather than silently implying global truth; widen/reset-scope and reindex-style recovery are only `provisional`.
- Evidence and gaps: accessibility evidence is canonical; quick-open-specific command identity, protected-path identity, and worked search-result examples are still missing or coupled to the broader palette path.

## Open, import, and restore

- Requirement ids: `FR-ENTRY-001`, `FR-ENTRY-002`, `FR-BOOT-006`, `MIG-INT-008`, `REL-CORE-003`, `SEC-TRUST-001`, `CERT-WS-001`, `A11Y-CORE-002`.
- Canonical artifacts: `cmd:workspace.open_folder`, `cmd:workspace.import_profile`, `cmd:workspace.restore_from_checkpoint`, the entry/restore schema and object model, and the shared accessibility task `corpus.accessibility.project_entry.open_import_restore`.
- Degraded and recovery coverage: `compatible_restore`, `missing_target`, `capability_disabled_by_policy`, and `basis_snapshot_drifted` all have named recovery hooks such as `open_without_restore`, `safe_mode`, `locate_missing_target`, `roll_back_import`, and the seeded repair hooks on the command rows.
- Evidence and gaps: the workspace entry/restore fixtures and invocation-session example give concrete proof; dedicated launcher-card and restore-prompt component contracts are still missing.

## Trust or permission prompt

- Requirement ids: `SEC-TRUST-001`, `REL-MUT-014`, `TOOL-CTX-002`, `A11Y-CORE-002`, `ARCH-UX-005`.
- Canonical artifacts: `trust.permission_prompt`, the shell interaction-safety contract, route-taxonomy/browser-handoff schemas, and the accessibility task `corpus.accessibility.trust.permission_prompt_recovery`.
- Degraded and recovery coverage: `review_required`, `restricted_path`, and `policy_blocked` stay distinct; continue-local and policy-review routes are named explicitly, and focus returns to the invoker or next safe surface.
- Evidence and gaps: the component fixture and trust-warning handoff example are canonical; the recovery commands referenced by the prompt are not yet in the command registry.

## Docs and help

- Requirement ids: `GOV-TRUTH-901`, `GOV-DATA-002`, `TOOL-CTX-002`, `A11Y-CORE-002`, `ARCH-UX-005`.
- Canonical artifacts: `cmd:docs.open_in_browser`, destination-descriptor contract and seeds, help-status badge vocabulary, browser-handoff packet schema, and the accessibility task `corpus.accessibility.docs.docs_help_reading`.
- Degraded and recovery coverage: `stale` freshness, `compatible_minor_drift`, cached-snapshot routes, and policy-blocked routes remain first-class states; browser fallback is governed by `cmd:docs.open_in_browser` and the descriptor fallback rules.
- Evidence and gaps: descriptor rows, the docs-help handoff packet example, and the command row are canonical; no dedicated docs-reader or help-badge component contract packet exists yet.

## Activity and banner surface

- Requirement ids: `TOOL-EVT-001`, `REL-SUPPORT-001`, `OPS-CLOUD-002`, `ARCH-UX-005`, `A11Y-CORE-002`.
- Canonical artifacts: `activity_center.durable_job_row`, the activity-event envelope schema, attention-routing taxonomy, interruptibility tiers, quiet-hours matrix, and the accessibility task `corpus.accessibility.execution.task_run_review`.
- Degraded and recovery coverage: durable rows expose `queued`, `blocked`, `failed`, and `degraded` state explicitly; banner escalation is governed by `tier_durable`, `tier_actionable`, and `tier_blocking_trust`.
- Evidence and gaps: durable-row contract and routing artifacts are canonical; there is no dedicated contextual-banner component contract and the row's recovery commands are not seeded in the command registry.

## Support or report handoff

- Requirement ids: `OPS-SUP-005`, `REL-SUPPORT-001`, `REL-REPAIR-015`, `GOV-DATA-002`, `TOOL-CTX-002`.
- Canonical artifacts: object-handoff schema and contract, support-bundle contract/schema, support-center concept, support-route examples, destination descriptors, and support-bundle example packets.
- Degraded and recovery coverage: handoff delivery states distinguish `saved_local_only`, `browser_handoff_blocked_retry_later`, `attached_by_reference`, and `manual_review_required`; recovery routes are `support_route.local_export_only.offline_follow_up`, `support_route.public_issue_tracker.supportability_issue`, and `support_bundle_by_reference`.
- Evidence and gaps: command-detail, trust-warning, docs-help, route, and bundle examples are canonical; dedicated accessibility coverage, component-contract coverage, and command-registry coverage for the handoff launcher are still missing.
