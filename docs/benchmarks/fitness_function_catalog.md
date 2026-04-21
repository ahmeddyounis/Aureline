# Protected fitness-function catalog

This document is the **normative** companion to the protected fitness-
function catalog at
[`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml).
It defines the closed vocabularies the catalog, the benchmark lab, the
journey harness, the release-evidence shiproom packets, and the
performance-council waiver log resolve against when they name a fitness
function.

If this document disagrees with the catalog, this document wins and the
catalog must be updated in the same change. Renaming any token defined
here is **breaking** and opens a decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml);
adding a value is additive-minor and lands in this document and the
catalog in the same change.

Companion artifacts:

- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — the machine-readable register.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — index row `fitness_function_catalog` names this document as the
  `overview_page`.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — the `benchmark_lab`, `aureline-render`, `aureline-buffer`,
  `aureline-vfs`, `shell_command_system`, and `governance_packets`
  lanes and the `performance_council` / `architecture_council` /
  `release_council` / `shiproom_executive_scope_review` decision
  forums referenced by every row.
- [`/docs/governance/benchmark_council_charter.md`](../governance/benchmark_council_charter.md)
  — the forum that waives, renews, and closes every protected
  fitness function here.
- [`/docs/governance/dri_map.md`](../governance/dri_map.md) — the
  authority table that pins waiver authority for protected fitness
  functions to **lane DRI plus performance council, with named
  expiry**.
- [`/docs/benchmarks/spike_metric_names.md`](./spike_metric_names.md)
  — the protected-path hook vocabulary whose bucket names are
  reused verbatim in every row's `metric.protected_path_bucket`.
- [`/docs/benchmarks/fixture_classes.md`](./fixture_classes.md) —
  the corpus vocabulary whose `protected_journeys` set every row's
  `protected_journeys` list quotes from.
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — the protected benchmark corpus manifest whose ids every row
  resolves against by `data_source.corpus_refs`.
- [`/artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml)
  — the benchmark-report and shiproom packet families that consume
  the `packet_export_shape.fitness_function_snapshot` block.

## 1. Why a fitness-function catalog exists

1. "Fast and safe" has to become a governed system property before more
   features exist, not after. Without a single protected register,
   every subsystem lane invents its own scoreboard, every release
   claim cites a private metric name, and every waiver is negotiated
   against a different set of assumptions. This catalog exists so
   that every protected speed / safety claim flows through one owner,
   one data source, one threshold, one waiver authority, and one
   review path.
2. Benchmark regressions have to be comparable across runs. The
   catalog freezes stable `rows[].id` identities so a release packet
   can say "the metric that failed is `ff.input_to_paint`" and every
   downstream consumer (benchmark lab, support bundle, claim
   manifest, shiproom packet) resolves it to the same row.
3. The benchmark council charter §3 names "protected-fitness
   waivers", "threshold changes", and "dispute resolution" as in
   scope. The council needs a catalog it can adjudicate against;
   this file makes the catalog auditable.
4. Later milestones will add rows for quick-open / command-dispatch,
   rename-with-preview, remote reconnect, AI first response, and the
   full TAD §3.4 journey set. Freezing the row shape now means those
   rows land against one register rather than a parallel scoreboard.

## 2. Scope

### In scope

- Closed vocabularies for: row status, architecture driver,
  architecture principle, protected journey, protected SLO family,
  threshold mode, data-source kind, waiver authority, review cadence.
- Metadata every row MUST carry (stable id, owner DRI, owning lane,
  co-owning lane when present, architecture drivers, architecture
  principles, protected journeys, protected SLO family, metric name
  and protected-path bucket, threshold mode, threshold body, data
  source with corpus refs and evidence-consumer channels, cadence
  with review and measurement cadence and re-baseline trigger,
  waiver authority, co-waiver authority, waiver rules, linked
  ADRs, linked decisions).
- Rules for `row_status`: when a row is seeded versus provisional
  versus not-yet-seeded.
- The export shape (`packet_export_shape.fitness_function_snapshot`)
  every shiproom / benchmark-report packet MUST carry when it cites a
  protected fitness function.
- Slice-index rules so rows can be read by driver, principle,
  journey, waiver authority, or SLO family without building a
  parallel scoreboard.

### Out of scope

- Numeric threshold values for any row. `to_be_set_by_benchmark_council`
  is the deliberate placeholder at this catalog revision; promotion to
  concrete numbers is reserved to the benchmark council charter §3 and
  governed by
  [`corpus_governance.md`](./corpus_governance.md) plus
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).
- Benchmark hardware baselines. Baselines are named as a decision slot
  in the benchmark council charter §3; enumerating them is deferred.
- Public-comparison framing, competitive-comparison rules, or
  publication of benchmark numbers as marketing claims. Those belong
  to [`public_comparison_rules.md`](./public_comparison_rules.md) and
  the claim-manifest process.
- Complete rows for every TAD §3.4 journey. The seven protected rows
  in this catalog plus three provisional rows are the foundations set;
  later milestones extend the register without reshaping the row
  schema.

## 3. Row-status vocabulary (frozen)

| `row_status`      | What it means                                                                                                 | Gate behaviour                                                          |
|-------------------|---------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|
| `seeded`          | Row names a concrete data source, has an owner DRI, and its waiver and review rules are explicit.             | Gates release evidence per its threshold and cadence.                   |
| `provisional`    | Row reserves a stable id and slot but does not yet gate anything; names a `completion_owner` for promotion.   | Reported in packets with `result: provisional`; does not block release. |
| `not_yet_seeded` | Row is declared but no home has landed; used only when a later ADR or decision row names the owner.           | Reported as `result: not_measured`; does not block release.             |

A `seeded` row that loses its data source MUST either move back to
`provisional` in the same change that removes the data source, or open
a protected-fitness waiver per §9 below.

## 4. Architecture-driver vocabulary (closed)

Every row MUST cite at least one driver. The set is the nine quality-
attribute drivers named by the Technical Architecture Document §3.2
"Quality attribute drivers". Adding a driver is an architecture-council
decision.

| Driver           | What it covers                                                                                 |
|------------------|------------------------------------------------------------------------------------------------|
| `latency`        | Hot-path responsiveness: warm start, first paint, input-to-paint, save pipeline latency.        |
| `scalability`    | Large repos, polyglot workspaces, hot-set indexing, background-service scaling.                 |
| `correctness`    | Source-fidelity, typed contracts, previewable writes, undo-class correctness, boundary truth.   |
| `recoverability` | Autosave journals, safe mode, extension quarantine, rebuildable caches, entry-restore fidelity. |
| `security`       | Workspace trust, sandboxing, policy bundles, signed artifacts, network governance.              |
| `portability`    | Human-readable configs, mirrorable artifacts, reproducible builds, open standards.              |
| `extensibility`  | Stable typed SDK contracts, conformance kit, command graph available to extensions.             |
| `supportability` | Project Doctor, support bundles, inspectors, trace IDs, headless diagnostics.                   |
| `accessibility`  | Accessibility bridge, keyboard-complete command graph, semantic surfaces, regression suite.     |

## 5. Architecture-principle vocabulary (closed)

Every row MUST cite at least one principle. The set is the ten
principles from the Technical Architecture Document §4.1 "Architecture
principles". Adding a principle is an architecture-council decision.

| Principle                                       | Short form (catalog id)                                |
|-------------------------------------------------|--------------------------------------------------------|
| Local-first shell, remote-capable services      | `local_first_shell_remote_capable_services`            |
| One command graph                               | `one_command_graph`                                    |
| One execution-context model                     | `one_execution_context_model`                          |
| One semantic workspace graph                    | `one_semantic_workspace_graph`                         |
| Heavy work out of process                       | `heavy_work_out_of_process`                            |
| Every write is reversible                       | `every_write_is_reversible`                            |
| Caches are disposable; user state is durable    | `caches_disposable_user_state_durable`                 |
| Open standards over bespoke lock-in             | `open_standards_over_bespoke_lock_in`                  |
| Optional services are additive                  | `optional_services_additive`                           |
| Accessibility and trust are system qualities    | `accessibility_and_trust_are_system_qualities`         |

## 6. Protected-journey vocabulary (aligned, not invented)

Every row MUST cite at least one `protected_journey`. The set is the
union of the ADR-0002 protected-path buckets frozen by
[`spike_metric_names.md`](./spike_metric_names.md) and the corpus-
specific tokens frozen by the corpus manifest's `protected_journeys`
block. A row MUST NOT invent a journey outside that set; widening the
set is additive-minor and lands in this document, in
[`fixture_classes.md`](./fixture_classes.md), and in the catalog in one
change.

Rationale: the catalog and the benchmark corpus manifest share one
journey vocabulary so fixtures cited by a row and the row that gates
them resolve to the same identity. A protected-path hook cannot be in
one bucket in the spike trace and a different bucket on a fitness row.

## 7. Protected-SLO-family vocabulary (closed)

Every row MUST cite exactly one `protected_slo_family`. The families
collect rows that share a user-visible budget, so release evidence can
group rows without re-deriving the mapping. The set is closed:

| Family                                | What it covers                                                                              |
|---------------------------------------|---------------------------------------------------------------------------------------------|
| `startup_and_first_useful_work`       | TAD §3.4 "Open repository to first useful navigation/edit"; warm-start and first-paint.     |
| `input_response`                      | TAD §3.4 "Keystroke to screen in active editor"; every ADR-0002 input-to-paint hook.        |
| `render_submission`                   | Per-frame GPU submission budget; the protected-path `render_submission` bucket.             |
| `quick_open_and_command_dispatch`     | TAD §3.4 "Quick open file/symbol/command"; reserved row-family slot, no rows at this rev.   |
| `save_and_filesystem_correctness`     | Buffer undo-class correctness + ADR-0006 save pipeline and conflict taxonomy.               |
| `recoverability_and_restore`          | Recovery journal write / restore and delivered-vs-advertised restore-fidelity rate.         |
| `remote_reconnect`                    | TAD §3.4 "Attach/reconnect to remote workspace"; reserved row-family slot.                  |
| `rename_with_preview`                 | TAD §3.4 "Rename with preview"; reserved row-family slot.                                   |
| `ai_first_response`                   | TAD §3.4 "AI explain selected failure"; reserved row-family slot.                           |
| `power_and_thermal_posture`           | TAD §8.7 power / thermal architecture — protected hot path under ThermalConstrained.        |
| `command_graph_parity`                | "One command graph" principle defended by a typed parity gate across every surface.         |
| `benchmark_lab_operational_health`    | Harness self-health. Not a user journey; a required precondition for every other row.       |

Reserved families (`quick_open_and_command_dispatch`, `remote_reconnect`,
`rename_with_preview`, `ai_first_response`, `render_submission`) carry
no rows at this revision but are admissible in the closed set so later
rows land without a vocabulary bump.

## 8. Threshold-mode and data-source vocabularies (closed)

### Threshold modes

| Mode                            | Meaning                                                                              |
|---------------------------------|--------------------------------------------------------------------------------------|
| `absolute_p50_and_p95`          | Two numeric latency targets on a named SLI.                                           |
| `absolute_p95_only`             | Tail-only target.                                                                     |
| `ratio_gate`                    | Pass / fail on a ratio (e.g. `restore_fidelity_match_rate`).                          |
| `counts_only`                   | Event counters; zero unless noted.                                                    |
| `boolean_gate`                  | Hard must-pass contract assertion (undo-class correctness, save-mode-no-demotion).   |
| `to_be_set_by_benchmark_council`| Provisional placeholder; mirrors the no-account scoreboard-seed convention.          |

### Data-source kinds

| Kind                                  | Where the measurement lives                                                              |
|---------------------------------------|------------------------------------------------------------------------------------------|
| `shell_spike_timing_trace`            | `schemas/traces/spike_timing.schema.json` + `artifacts/traces/examples/`.                |
| `benchmark_lab_harness_run`           | `crates/aureline-bench` harness output.                                                  |
| `corpus_fixture_measurement`          | Direct fixture-backed measurement against a corpus id.                                   |
| `support_bundle_summary`              | Evidence-consumer channel from the corpus manifest.                                      |
| `entry_restore_record`                | `schemas/workspace/entry_and_restore_result.schema.json`.                                |
| `governance_packet_self_audit`        | Benchmark-report packet family reading itself against its schema.                        |
| `to_be_wired_by_benchmark_council`    | Provisional placeholder; the row is reserved but not yet measured.                       |

## 9. Waiver authority and review cadence

### Waiver authority

Per [`dri_map.md`](../governance/dri_map.md) §Authority for narrowing,
waivers, and re-baselining: **opening or renewing a waiver on a
protected fitness function requires the lane DRI plus the performance
council, with a named expiry.**

Every row carries both:

- `waiver_authority: performance_council` — the forum that opens,
  renews, and closes the waiver; benchmark-council
  [charter](../governance/benchmark_council_charter.md) §3 names this
  as "the only forum that may waive a protected fitness function, with
  a named expiry and a documented correction programme".
- `co_waiver_authority: <lane_id>` — the co-signing lane DRI from
  [`ownership_matrix.scorecard_lane_index`](../../artifacts/governance/ownership_matrix.yaml).
  The forum cannot waive a row without the co-signing lane.

A waiver on `ff.vfs_save_conflict_handling` additionally requires the
`release_council` on the escalation path, because a demotion of
compare-before-write or a silent atomic-to-in-place demotion would
touch the release-evidence claim manifest. That is why the row's
`waiver_rules.escalation_path` names the release council explicitly.

### Review cadence

Values are drawn from
[`control_artifact_index.yaml#review_cadences`](../../artifacts/governance/control_artifact_index.yaml)
so tooling validates every cadence against one closed set.

| Cadence            | What it means on a fitness row                                                    |
|--------------------|-----------------------------------------------------------------------------------|
| `each_change`      | Re-evaluated on every pull request that could affect the row; used on `ff.benchmark_lab_health`. |
| `per_milestone`    | Reviewed at milestone boundaries alongside the scorecard; the default.             |
| `each_release`     | Consulted during release-evidence assembly; used on every seeded row's `measurement_cadence`. |

### Expiry and review rules

Every row's `waiver_rules` block carries:

- `default_expiry_window_days` — maximum time a waiver may be open
  before renewal. Default is 90 days for renderer / startup rows, 60
  days for input and buffer rows, 45 days for the VFS save-pipeline
  row, 30 days for the benchmark-lab-health row, 180 days for
  provisional rows. Shorter windows live closer to the release; the
  save-pipeline row is the tightest because a waiver there directly
  touches data correctness.
- `renewal_requires_correction_program` — true on every seeded row; a
  waiver may be renewed only once under a named correction program
  before it is closed or escalated. Per [`dri_map.md`](../governance/dri_map.md)
  §Blocker aging: "Recurring waiver on the same protected lane
  (second renewal) — Convert to a tracked correction program; waivers
  do not become the steady state."
- `escalation_path` — ordered list of forums a refused or expired
  waiver routes to. Always starts at the performance council and
  terminates at the shiproom / executive scope review.

## 10. Export format for shiproom / review packets

Every benchmark-report and shiproom packet that cites a protected
fitness function MUST carry a `fitness_function_snapshot` block whose
shape is frozen at
[`packet_export_shape.fitness_function_snapshot`](../../artifacts/bench/fitness_function_catalog.yaml)
in the catalog. Required fields:

- `row_id` — resolves to a `rows[].id` in the catalog.
- `catalog_revision` — the catalog's `catalog_revision` at packet
  assembly time.
- `measured_on` — `YYYY-MM-DD` on which the measurement was captured.
- `measurement_run_id` — opaque run identifier from the harness.
- `data_source_ref` — path or channel id resolved at run time.
- `corpus_refs` — list of corpus ids from
  [`corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml).
- `host_platform_class` — `host_independent`, `host_dependent`, or
  `host_os_pinned` per
  [`fixture_classes.md`](./fixture_classes.md) §7.
- `host_os` and `rustc_target_triple` — required when
  `host_platform_class != host_independent`; match the corpus-
  manifest rule so host-dependent bytes and measurements are
  recorded against the same fields.
- `threshold_at_measurement` — copy of the row's threshold at packet
  time. The copy is made so a later catalog revision that tightens
  the threshold does not retroactively rewrite the packet.
- `result` — `pass`, `fail`, `not_measured`, `waived`, or
  `provisional`.
- `waiver_refs` — list of waiver ids from
  [`ownership_matrix.yaml#waivers`](../../artifacts/governance/ownership_matrix.yaml);
  mandatory when `result == waived`.
- `expires_on` — required when `result == waived`.

Optional fields: `notes`, `evidence_consumer_channels_touched`. A
packet that cites a row without a conforming snapshot block is a
validation failure; the benchmark-report and shiproom packet schemas
under
[`schemas/governance/`](../../schemas/governance/) validate against
this shape.

## 11. Per-row narrative

Each row in the catalog carries its own `notes` block. This section
is the human-readable overview of what each row defends and why it
is shaped the way it is; consult the YAML for authoritative fields.

- **`ff.warm_start_to_first_paint`** — defends warm start on the
  claimed hardware matrix. Data source is the shell-spike timing
  trace schema; corpus refs include the workflow scenario
  `corpus.workflow.startup_warm_to_first_paint` and its reference
  workspace `corpus.reference.micro_local_folder`. Waiver rules are
  the 90-day renderer default. Linked to `D-0001` and
  [ADR-0002](../adr/0002-renderer-text-stack-and-shaping-fallback.md).
- **`ff.first_paint`** — defends first-paint latency per surface,
  including reopen from a hidden state. Shares data source and
  waiver rules with the warm-start row but is kept separate because
  first-paint regressions can occur without a warm-start
  regression (and vice versa). A waiver additionally requires the
  accessibility / input review lane liaison because
  `accessibility_tree_update` is on the same protected hot-path
  list.
- **`ff.input_to_paint`** — defends every ADR-0002 input-to-paint
  hook under one SLI. Tighter expiry window (60 days) than
  startup / first-paint because input latency regressions are
  user-visible on every keystroke. Covers `scroll_frame`,
  `caret_move`, `selection_change`, `ime_composition_update`,
  `reflow_line_range`, and `multi_monitor_scale_change`.
- **`ff.buffer_operations`** — defends both buffer-operation
  latency and undo-class correctness. The correctness half is a
  `boolean_gate`, not a latency number: a `refactor_multi_file`
  operation that is fast but misclassifies itself as
  `text_edit` is release-blocking. Linked to `D-0002`,
  [ADR-0003](../adr/0003-buffer-undo-large-file.md), and
  [`undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml).
- **`ff.vfs_save_conflict_handling`** — defends the ADR-0006 save
  pipeline. Compare-before-write on the strongest identity token
  is the correctness floor; atomic-replace is the preferred
  commit mode; every failure reason in the ADR-0006 closed set
  must resolve to a typed outcome. Tightest expiry window (45
  days) and widest escalation path — the release council is on
  the path because a waiver here affects the release-evidence
  claim manifest.
- **`ff.benchmark_lab_health`** — defends the harness itself.
  Cadence is `each_change` (not `per_milestone`) because a broken
  harness invalidates every other row on this catalog until it is
  fixed. Result feeds the benchmark-report self-audit and, through
  it, the release-evidence claim manifest. The row exists because
  a fitness catalog that cannot audit itself is not an evidence
  source.
- **`ff.power_thermal_posture`** (provisional) — reserves the
  identity the TAD §8.7 power / thermal architecture will
  eventually gate against: under ThermalConstrained and
  ProtectCore the hot path stays inside its budget, and the
  declared workloads shed in the declared order. No ADR has been
  ratified; the row is provisional until one lands.
- **`ff.restore_fidelity`** (provisional) — reserves the
  `restore_fidelity_match_rate` name the onboarding measurement
  plan already uses, and slots against the four restore-fidelity
  labels (`exact`, `compatible`, `layout_only`, `manual_review`)
  frozen by the portable-profile and entry-restore object models.
  The contracts are frozen; the harness that measures delivered-
  vs-advertised fidelity is not yet wired.
- **`ff.command_parity`** (provisional) — reserves the typed
  parity gate that defends the "one command graph" principle
  across every surface: palette, menu, context menu, keybinding
  layer, CLI, AI-tool surface, automation recipe. Command-
  descriptor contracts are frozen; the parity harness awaits a
  command-graph ADR.

## 12. Slicing the catalog

The catalog is sliced by five axes, all derived from row fields and
carried as a minimal `slices:` block in the YAML so a consumer does
not need to walk every row to build a view:

- `slices.by_architecture_driver` — which rows defend which driver.
- `slices.by_architecture_principle` — which rows defend which
  principle.
- `slices.by_protected_journey` — which rows fire on which protected-
  path bucket or corpus-journey token.
- `slices.by_waiver_authority` — which rows route through which forum.
  At this revision every row routes through `performance_council`;
  the other forums are reserved so later rows that need a different
  authority (for example, security-council rows) land without
  reshaping the vocabulary.
- `slices.by_protected_slo_family` — which rows share a user-visible
  budget family.

Slicing rules:

- Every row in `rows:` appears under at least one key in
  `by_architecture_driver`, at least one key in
  `by_architecture_principle`, at least one key in
  `by_protected_journey`, exactly one key in
  `by_waiver_authority`, and exactly one key in
  `by_protected_slo_family`.
- A slice that contains no rows MUST still appear with an empty list
  so the closed vocabulary is exhaustive.
- A row is NEVER added to a slice without a matching field on the
  row; the slice block is a view, not an index, and it regenerates
  from the rows.

## 13. How each role uses the catalog

### Engineering

- Before introducing a new protected metric — a register, manifest,
  review packet, or machine-readable surface definition that claims
  a speed or safety budget — look for an existing row in the catalog.
  If the metric already has a canonical row, extend that row; do not
  mint a parallel one.
- When a pull request changes anything under a `data_source.schema`
  or `data_source.example_artifacts` path named in the catalog,
  update the corresponding row's `notes` or `data_source` in the
  same change.
- Review cadence `each_change` on a row means the row must be
  re-consulted on every pull request that could affect it. The only
  `each_change` row at this revision is `ff.benchmark_lab_health`.

### Design

- Accessibility and input-response rows (`ff.first_paint`,
  `ff.input_to_paint`, `ff.command_parity`) cite the accessibility
  driver and the "accessibility and trust are system qualities"
  principle; design changes that touch `accessibility_tree_update`
  or the keyboard-complete command graph route through those rows
  as evidence owners.

### Quality engineering

- The catalog is the single place quality-engineering rows land.
  New benchmark corpora, qualification gates, and fitness bars
  MUST either extend an existing row (adding a corpus ref, updating
  a threshold once the benchmark council promotes one) or land a
  new row under the same schema.
- Benchmark-report and shiproom packets cite rows by `rows[].id`;
  the packet schemas validate that ids resolve against this catalog.

### Docs and public-truth

- Any doc claim that a downstream consumer might rely on
  ("Aureline feels fast on thermally constrained laptops", "restore
  fidelity matches what was advertised") MUST cite the owning row
  via the claim-manifest packet family. Provisional rows may be
  cited as "a contract is reserved but not yet gated"; seeded rows
  may be cited as "gated by the benchmark council against this
  corpus".

### Release

- Release-evidence assembly MUST iterate every seeded row and
  emit a `fitness_function_snapshot` block. A row whose
  `row_status: seeded` produces `result: not_measured` is a
  release-evidence gap, not a pass.
- Waiver refs cited on a row MUST resolve to an active waiver id in
  [`ownership_matrix.yaml#waivers`](../../artifacts/governance/ownership_matrix.yaml).

### Support

- Support bundles that summarise performance (`support_bundle.performance_summary`,
  `support_bundle.crash_recovery_summary`) MAY cite row ids so a
  customer-facing bundle resolves to the same protected metric the
  release evidence does. The bundle never invents a parallel metric
  name.

## 14. Change policy

- **Additive-minor** — new row id, new threshold mode, new data-
  source kind, new protected-SLO-family value, new evidence-consumer
  channel — lands in this document and the catalog in the same
  change. The change cites the journey, corpus, or evidence family
  that motivates it.
- **Breaking** — repurposing an id, renaming a vocabulary token,
  switching a row between `threshold_modes`, demoting a `seeded`
  row to `provisional` — opens a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- **Benchmark-council authority** — promoting a threshold from
  `to_be_set_by_benchmark_council` to a concrete numeric bar,
  retiring a row, or opening a waiver is a benchmark-council decision
  per the charter §3. The decision is recorded in the performance-
  council packet family, not inline here.

## 15. What this catalog is not

- It is **not** the detailed governance workflow. That lives in
  [`decision_workflow.md`](../governance/decision_workflow.md).
- It is **not** a hardware-baseline register. Hardware baselines are
  named as a decision slot in the benchmark-council charter §3;
  enumerating them is deferred.
- It is **not** a public benchmark-results page. Publication rules
  are owned by [`public_comparison_rules.md`](./public_comparison_rules.md).
- It is **not** a substitute for the ownership matrix. The matrix
  defines who owns each lane; this catalog defines which fitness
  functions the lanes gate. Lane ids here always resolve back to
  [`ownership_matrix.scorecard_lane_index`](../../artifacts/governance/ownership_matrix.yaml).

## 16. Change discipline

- Adding a fitness row requires: a row in
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml),
  at least one architecture driver, at least one architecture
  principle, at least one protected journey, exactly one protected
  SLO family, a named owner DRI, a lane id from the ownership
  matrix, a waiver authority, a review cadence, waiver rules, and a
  `data_source` (or `to_be_wired_by_benchmark_council` with a
  `completion_owner`). All in the same change. The `slices:` block
  is regenerated in the same change so the view matches the rows.
- Retiring a row requires: moving its `row_status` to reflect the
  retirement and leaving the row in place with a note. Rows are
  not deleted, so the audit trail of "this gate existed and was
  retired" survives.
- When this document and the YAML catalog disagree, the document
  wins for vocabulary and the YAML is updated in the same change.
  Tooling reads the YAML, so the document must be kept current.
