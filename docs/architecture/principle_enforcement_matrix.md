# Architecture principle enforcement matrix

This document turns Aureline’s architecture principles into **reviewable** and
**CI-checkable** expectations. The principles are not new requirements; they are
the decision summary from the Technical Architecture Document. This matrix makes
them actionable by naming, per principle:

- the protected paths and lanes where it matters most,
- the invariants that must hold,
- common anti-patterns and explicit failure conditions,
- the controlling review forum and waiver authority, and
- the machine checks and evidence packet families that defend the principle.

## Source of truth

- Principles: `.t2/docs/Aureline_Technical_Architecture_Document.md` §4.1.
- Principle ids (short forms): `docs/benchmarks/fitness_function_catalog.md` §5.
- Machine hooks introduced here:
  - `artifacts/architecture/principle_checks.yaml`
  - `artifacts/architecture/principle_violation_examples.yaml`

If the prose in this document disagrees with the TAD’s principle list, the TAD
wins. If this document disagrees with the machine hooks, the machine hooks win
for CI validation and this document must be updated in the same change.

## Degradation vs contradiction (how to reason about “violations”)

Architecture principles are enforced in two different ways:

1) **Temporary narrowing (acceptable with explicit disclosure).**  
   A change may *temporarily* narrow a claimed posture as long as it:
   - keeps the invariant boundary explicit (truthful labels, no silent drift),
   - records the narrowing in the owning artifact (ledger, claim, profile, or
     contract row), and
   - routes through the correct review forum (and waiver packet when required).

2) **Release-blocking contradiction (not acceptable as an implicit trade).**  
   A change is a contradiction when it would make the principle untrue for a
   protected path or a claimed posture, or when it introduces an irreversible or
   unreviewable shortcut (for example: blocking I/O on the hot path, private
   command identity, silent trust/permission widening). Contradictions are
   treated as merge blockers on protected lanes unless an explicit waiver packet
   scopes and time-boxes the exception.

Worked examples live in `artifacts/architecture/principle_violation_examples.yaml`.

## Enforcement matrix (summary)

This table is a **routing summary**. Full invariants, controls, and failure
conditions live in `artifacts/architecture/principle_checks.yaml`.

| Principle id | Controlling forum(s) | Primary machine gate(s) | Evidence packet families |
|---|---|---|---|
| `local_first_shell_remote_capable_services` | `architecture_council`, `performance_council` | `tools/check_protected_dependencies.py`, protected-fitness catalog | `verification_packet`, `benchmark_report` |
| `one_command_graph` | `architecture_council` | `tools/ci/validate_contract_artifacts.py` (command parity seed) | `verification_packet`, `benchmark_report` |
| `one_execution_context_model` | `architecture_council` | frozen-surface / schema-change gate | `verification_packet` |
| `one_semantic_workspace_graph` | `architecture_council` | contract + interface-freeze lanes (schema + truth vocabularies) | `verification_packet` |
| `heavy_work_out_of_process` | `architecture_council` | process-placement + protected-dependency gate | `verification_packet` |
| `every_write_is_reversible` | `architecture_council`, `release_council` | frozen-surface + claim-publication posture checks | `verification_packet`, `waiver_packet` |
| `caches_disposable_user_state_durable` | `architecture_council` | save/restore + state contract gates; protected-path ledgers | `verification_packet`, `benchmark_report` |
| `open_standards_over_bespoke_lock_in` | `architecture_council`, `compatibility_ecosystem_review` | standards matrix + deviation decision workflow | `compatibility_report`, `verification_packet` |
| `optional_services_additive` | `product_scope_review`, `architecture_council` | deployment profile + residual dependency ledger | `compatibility_report`, `verification_packet` |
| `accessibility_and_trust_are_system_qualities` | `accessibility_review`, `security_trust_review` | accessibility contract packs + trust-boundary artifacts | `design_packet`, `verification_packet` |

## Principles (canonical list with enforcement hooks)

Each section below is intentionally scoped to: rationale, protected paths, core
invariants, and “what fails” signals. It is not a substitute for subsystem ADRs
or cards; it is the shared reviewer checklist so those documents do not need to
duplicate governance prose.

### 1) Local-first shell, remote-capable services (`local_first_shell_remote_capable_services`)

**Rationale**
- Preserve “usable before the world is ready”: the shell stays responsive even
  when remote compute, indexing, or services are slow/unavailable.

**Protected paths (examples)**
- Shell + renderer hot path packages and monitored modules (see
  `artifacts/architecture/protected_path_dependency_rules.yaml`).

**Expected invariants**
- No blocking filesystem/network/process work on protected interaction paths.
- Remote-only capability is disclosed as such; local interaction does not stall
  waiting for remote readiness.

**Common anti-patterns**
- `rp.blocking_io_on_hot_path`
- `rp.forced_account_or_network_gate_before_local_work`
- `rp.optional_service_on_core_path`

**Explicit failure conditions**
- A protected module imports a forbidden sentinel (`std::fs`, `std::net`,
  subprocess launch) without routing through an allowed worker boundary.
- A core local workflow requires a hosted service with no explicit degraded path.

### 2) One command graph (`one_command_graph`)

**Rationale**
- Every surface (palette, menu, keybindings, CLI, automation, AI tools) must
  resolve to one typed command identity so behavior stays reviewable and
  discoverable.

**Protected paths (examples)**
- Command descriptor contracts and projection fixtures; command parity seed.

**Expected invariants**
- A command has one stable id, one argument contract, and one result contract.
- Surfaces may narrow availability only via typed disabled reasons; they must
  not mint private command ids.

**Common anti-patterns**
- `rp.private_command_paths_bypass_descriptors`

**Explicit failure conditions**
- A surface projects a command id/label/preview posture/result contract that
  disagrees with the canonical descriptor without an explicit narrowing record.

### 3) One execution-context model (`one_execution_context_model`)

**Rationale**
- Tasks, terminals, tests, debug sessions, and AI actions must resolve through
  one inspectable toolchain/environment model so provenance and supportability
  survive across UI and headless use.

**Protected paths (examples)**
- `schemas/runtime/execution_context.schema.json` and the execution-context ADR.

**Expected invariants**
- Launch surfaces must emit (or be able to emit) an execution-context record
  that explains toolchain, target, activators, and policy decisions.
- “Fallback” behavior is labeled; ad-hoc environment resolution is not hidden.

**Common anti-patterns**
- `rp.hidden_retry_loop_past_deadline`
- `rp.hidden_replay_or_mutation_on_restore`

**Explicit failure conditions**
- A new launch-capable surface bypasses execution-context truth and cannot
  produce a supportable, inspectable provenance record.

### 4) One semantic workspace graph (`one_semantic_workspace_graph`)

**Rationale**
- Search, navigation, refactors, review, and AI context must not maintain
  contradictory private truth models.

**Protected paths (examples)**
- Search/navigation graph contracts and result-truth label vocabularies.

**Expected invariants**
- One shared truth vocabulary (freshness, confidence, provenance) is used across
  consumers; “exact” claims require appropriate evidence.

**Common anti-patterns**
- `rp.unbounded_workspace_scan_on_core_path`

**Explicit failure conditions**
- A feature introduces a private semantic store that can disagree with shared
  graph truth without an explicit experimental label and sunset plan.

### 5) Heavy work out of process (`heavy_work_out_of_process`)

**Rationale**
- Crash containment, latency budgets, and isolation require heavyweight work to
  run outside the shell process.

**Protected paths (examples)**
- Process placement and runtime host-class maps.

**Expected invariants**
- Protected shell and renderer paths do not host heavyweight language/debug/AI
  execution.

**Common anti-patterns**
- `rp.background_work_starves_input`

**Explicit failure conditions**
- A protected shell-plane package takes a dependency that implies heavyweight
  execution in-process, violating the plane/placement rules.

### 6) Every write is reversible (`every_write_is_reversible`)

**Rationale**
- Trust is earned by previewable, attributable changes with a clear undo and
  rollback story.

**Protected paths (examples)**
- Mutation lineage contracts, preview runtime contract, and save/restore rules.

**Expected invariants**
- Multi-file edits (refactors, AI edits, migrations) produce a reviewable patch
  or checkpoint and an undo-class story.

**Common anti-patterns**
- `rp.unreviewable_write_without_journal_or_preview`
- `rp.silent_contract_downgrade`

**Explicit failure conditions**
- A write-bearing feature bypasses preview/checkpoint/journal requirements and
  cannot be reversed without manual archaeology.

### 7) Caches are disposable; user state is durable (`caches_disposable_user_state_durable`)

**Rationale**
- Portability and recovery depend on durable state being human-readable and
  rebuildable; caches must never become hidden sources of truth.

**Protected paths (examples)**
- Profile/state schemas and cache contracts; save/restore fitness rows.

**Expected invariants**
- Durable state is versioned and recoverable; derived state is rebuildable and
  safe to delete.

**Explicit failure conditions**
- A feature stores user-authored durable state only in derived/cache storage or
  uses a cache as a correctness dependency without versioning and rebuild rules.

### 8) Open standards over bespoke lock-in (`open_standards_over_bespoke_lock_in`)

**Rationale**
- Interoperability and ecosystem trust require standard-shaped formats where
  feasible, with explicit deviation records when not.

**Protected paths (examples)**
- Standards/interchange matrix and deviation ADR workflow.

**Expected invariants**
- Any standard-compatibility claim maps to a row in the standards matrix or a
  recorded deviation decision.

**Explicit failure conditions**
- A new externally-consumable format is introduced without a standards-matrix
  row and without an explicit deviation path.

### 9) Optional services are additive (`optional_services_additive`)

**Rationale**
- Hosted services may enhance the product, but they must not become hidden
  prerequisites for core workflows.

**Protected paths (examples)**
- Deployment profile truth, residual dependency ledger, and optional-service API
  inventories.

**Expected invariants**
- Core local workflows remain usable under “no service reachable” profiles.
- Service impairment uses typed dependency classes and explicit degrade posture.

**Common anti-patterns**
- `rp.optional_service_on_core_path`
- `rp.forced_account_or_network_gate_before_local_work`

**Explicit failure conditions**
- A core workflow becomes blocked on an optional service in a profile where the
  dependency is `forbidden` or `not_applicable_structural`.

### 10) Accessibility and trust are system qualities (`accessibility_and_trust_are_system_qualities`)

**Rationale**
- Accessibility, privacy, provenance, and recovery are platform qualities; they
  cannot be retrofitted after the shell ships.

**Protected paths (examples)**
- Accessibility contract pack; trust-boundary artifacts and capability controls.

**Expected invariants**
- Keyboard-complete routing through the command graph for launch-bearing
  surfaces.
- Trust and permission boundaries are explicit and inspectable; no silent
  widening or undisclosed egress.

**Common anti-patterns**
- `rp.silent_target_rebind_or_widening`
- `rp.provider_egress_before_policy`

**Explicit failure conditions**
- A launch-critical surface ships without keyboard/screen-reader parity.
- A feature performs network/provider egress before trust/policy gates or hides
  provenance and auditability.

